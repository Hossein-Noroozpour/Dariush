use winapi::shared::{dxgi, dxgiformat as dxfmt, dxgitype as dxtp, minwindef as mwd,
                     winerror as werr};
use winapi::um::{d3d11 as d11, d3dcommon as d3};
use winapi::Interface as WinInterface;
use std::ptr::null_mut;
use std::mem::{transmute, transmute_copy, zeroed};
use super::string::wchar_arr_to_string;
use super::window::Window;

pub struct RenderEngine {
    swapchain: &'static mut dxgi::IDXGISwapChain,
    device: &'static mut d11::ID3D11Device,
    context: &'static mut d11::ID3D11DeviceContext,
    main_rtv: &'static mut d11::ID3D11RenderTargetView,
    main_dsb: &'static mut d11::ID3D11Texture2D,
    main_dss: &'static mut d11::ID3D11DepthStencilState,
    main_dsv: &'static mut d11::ID3D11DepthStencilView,
    main_viewport: d11::D3D11_VIEWPORT,
    raster: &'static mut d11::ID3D11RasterizerState,
    shadow_rtv: &'static mut d11::ID3D11RenderTargetView,
    shadow_dsb: &'static mut d11::ID3D11Texture2D,
    shadow_dsv: &'static mut d11::ID3D11DepthStencilView,
    shadow_viewport: d11::D3D11_VIEWPORT,
    graphic_memory_size: u64,
}

impl RenderEngine {
    pub fn new() -> Self {
        unsafe {
            RenderEngine {
                swapchain: transmute(0usize),
                device: transmute(0usize),
                context: transmute(0usize),
                main_rtv: transmute(0usize),
                main_dsb: transmute(0usize),
                main_dss: transmute(0usize),
                main_dsv: transmute(0usize),
                main_viewport: d11::D3D11_VIEWPORT {
                    TopLeftX: 0f32,
                    TopLeftY: 0f32,
                    Width: 0f32,
                    Height: 0f32,
                    MinDepth: 0f32,
                    MaxDepth: 0f32,
                },
                raster: transmute(0usize),
                shadow_rtv: transmute(0usize),
                shadow_dsb: transmute(0usize),
                shadow_dsv: transmute(0usize),
                shadow_viewport: d11::D3D11_VIEWPORT {
                    TopLeftX: 0f32,
                    TopLeftY: 0f32,
                    Width: 0f32,
                    Height: 0f32,
                    MinDepth: 0f32,
                    MaxDepth: 0f32,
                },
                graphic_memory_size: 0,
            }
        }
    }

    pub fn is_initialized(&self) -> bool {
        let s: usize = unsafe { transmute_copy(self.swapchain) };
        s == 0
    }

    pub fn initialize(&mut self, window: &Window) {
        let mut factory: &mut dxgi::IDXGIFactory = unsafe { transmute(0usize) };
        let mut adapter: &mut dxgi::IDXGIAdapter = unsafe { transmute(0usize) };
        let mut adapter_output: &mut dxgi::IDXGIOutput = unsafe { transmute(0usize) };
        let mut numerator: mwd::UINT = 0;
        let mut denominator: mwd::UINT = 0;
        hr_check!(dxgi::CreateDXGIFactory(
            &dxgi::IDXGIFactory::uuidof(),
            transmute(&mut factory)
        ));
        let mut adapter_index: mwd::UINT = 0;
        'adapter_loop: loop {
            if unsafe {
                (*factory).EnumAdapters(adapter_index, transmute(&mut adapter))
                    == werr::DXGI_ERROR_NOT_FOUND
            } {
                break;
            }
            let mut adapter_desc: dxgi::DXGI_ADAPTER_DESC = unsafe { zeroed() };
            hr_check!((*adapter).GetDesc(&mut adapter_desc));
            self.graphic_memory_size = adapter_desc.DedicatedVideoMemory as u64;
            println!(
                "Video Card Description: {}",
                wchar_arr_to_string(&adapter_desc.Description)
            );
            let mut adpout_index: mwd::UINT = 0;
            loop {
                if 0usize == unsafe { transmute_copy(adapter_output) } {
                    unsafe {
                        adapter_output.Release();
                        adapter_output = transmute(0usize);
                    }
                }
                if unsafe { (*adapter).EnumOutputs(adpout_index, transmute(&mut adapter_output)) }
                    == werr::DXGI_ERROR_NOT_FOUND
                {
                    break;
                }
                adpout_index += 1;
                let mut num_modes: mwd::UINT = 0;
                if unsafe {
                    (*adapter_output).GetDisplayModeList(
                        dxfmt::DXGI_FORMAT_R8G8B8A8_UNORM,
                        1,
                        &mut num_modes,
                        null_mut(),
                    )
                } < 0
                {
                    continue;
                }
                if num_modes == 0 {
                    continue;
                }
                let mut display_mode_list =
                    vec![unsafe { zeroed::<dxtp::DXGI_MODE_DESC>() }; num_modes as usize];
                if unsafe {
                    (*adapter_output).GetDisplayModeList(
                        dxfmt::DXGI_FORMAT_R8G8B8A8_UNORM,
                        1,
                        &mut num_modes,
                        display_mode_list.as_mut_ptr(),
                    )
                } < 0
                {
                    continue;
                }
                for i in 0..num_modes as usize {
                    if display_mode_list[i].Width == window.screen_width as mwd::UINT {
                        if display_mode_list[i].Height == window.screen_height as mwd::UINT {
                            numerator = display_mode_list[i].RefreshRate.Numerator;
                            denominator = display_mode_list[i].RefreshRate.Denominator;
                        }
                    }
                }
                break 'adapter_loop;
            }
            adapter_index += 1;
            unsafe {
                adapter.Release();
                adapter = transmute(0usize);
            }
        }
        if 0usize == unsafe { transmute_copy(adapter_output) } {
            unsafe {
                adapter_output.Release();
                adapter_output = transmute(0usize);
            }
        }
        if 0usize == unsafe { transmute_copy(adapter) } {
            unsafe {
                adapter.Release();
                adapter = transmute(0usize);
            }
        }
        if 0usize == unsafe { transmute_copy(factory) } {
            unsafe {
                factory.Release();
                factory = transmute(0usize);
            }
        }
        let mut swap_chain_desc: dxgi::DXGI_SWAP_CHAIN_DESC = unsafe { zeroed() };
        swap_chain_desc.BufferCount = 1;
        swap_chain_desc.BufferDesc.Width = window.screen_width as mwd::UINT;
        swap_chain_desc.BufferDesc.Height = window.screen_height as mwd::UINT;
        swap_chain_desc.BufferDesc.Format = dxfmt::DXGI_FORMAT_R8G8B8A8_UNORM;
        swap_chain_desc.BufferDesc.RefreshRate.Numerator = numerator;
        swap_chain_desc.BufferDesc.RefreshRate.Denominator = denominator;
        swap_chain_desc.BufferUsage = dxtp::DXGI_USAGE_RENDER_TARGET_OUTPUT;
        swap_chain_desc.OutputWindow = window.window;
        swap_chain_desc.SampleDesc.Count = 1;
        swap_chain_desc.Windowed = 0;
        swap_chain_desc.BufferDesc.ScanlineOrdering = dxtp::DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED;
        swap_chain_desc.BufferDesc.Scaling = dxtp::DXGI_MODE_SCALING_UNSPECIFIED;
        swap_chain_desc.SwapEffect = dxgi::DXGI_SWAP_EFFECT_DISCARD;
        swap_chain_desc.Flags = 0;
        #[cfg(debug_assertions)]
        let device_flag: mwd::UINT = d11::D3D11_CREATE_DEVICE_DEBUG;
        #[cfg(not(debug_assertions))]
        let device_flag: mwd::UINT = d11::D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT;
        let feature_level: d3::D3D_FEATURE_LEVEL = d3::D3D_FEATURE_LEVEL_11_0;
        #[cfg(debug_assertions)]
        let mut driver_type: d3::D3D_DRIVER_TYPE = d3::D3D_DRIVER_TYPE_HARDWARE;
        #[cfg(not(debug_assertions))]
        let driver_type: d3::D3D_DRIVER_TYPE = d3::D3D_DRIVER_TYPE_HARDWARE;
        #[cfg(debug_assertions)]
        {
            if unsafe {
                d11::D3D11CreateDevice(
                    null_mut(),
                    driver_type,
                    null_mut(),
                    device_flag,
                    &feature_level,
                    1,
                    d11::D3D11_SDK_VERSION,
                    transmute(&mut self.device),
                    null_mut(),
                    null_mut(),
                )
            } < 0
            {
                driver_type = d3::D3D_DRIVER_TYPE_REFERENCE;
                hr_check!(d11::D3D11CreateDevice(
                    null_mut(),
                    driver_type,
                    null_mut(),
                    device_flag,
                    &feature_level,
                    1,
                    d11::D3D11_SDK_VERSION,
                    transmute(&mut self.device),
                    null_mut(),
                    null_mut()
                ));
            }
        }
        #[cfg(not(debug_assertions))]
        hr_check!(d3::D3D11CreateDevice(
            null_mut(),
            driver_type,
            null_mut(),
            device_flag,
            &feature_level,
            1,
            d3::D3D11_SDK_VERSION,
            &mut self.device,
            null_mut(),
            null_mut()
        ));
        for i in 0..d11::D3D11_MAX_MULTISAMPLE_SAMPLE_COUNT {
            swap_chain_desc.SampleDesc.Count = i;
            let mut sample_quality: mwd::UINT = 0;
            unsafe {
                self.device.CheckMultisampleQualityLevels(
                    swap_chain_desc.BufferDesc.Format,
                    i,
                    &mut sample_quality,
                );
            }
            if sample_quality > 0 {
                sample_quality -= 1;
                swap_chain_desc.SampleDesc.Quality = sample_quality;
                swap_chain_desc.SampleDesc.Count = i;
                println!(
                    "Multisampling selection count: {},  quality: {}",
                    i, sample_quality
                );
                break;
            }
        }
        unsafe {
            self.device.Release();
            self.device = transmute(0usize);
        }
        hr_check!(d11::D3D11CreateDeviceAndSwapChain(
            null_mut(),
            driver_type,
            null_mut(),
            device_flag,
            &feature_level,
            1,
            d11::D3D11_SDK_VERSION,
            &swap_chain_desc,
            transmute(&mut self.swapchain),
            transmute(&mut self.device),
            null_mut(),
            transmute(&mut self.context)
        ));
        let mut back_buffer_ptr: &mut d11::ID3D11Texture2D = unsafe { transmute(0usize) };
        hr_check!(self.swapchain.GetBuffer(
            0,
            &d11::ID3D11Texture2D::uuidof(),
            transmute(&mut back_buffer_ptr)
        ));
        hr_check!(self.device.CreateRenderTargetView(
            transmute_copy(back_buffer_ptr),
            null_mut(),
            transmute(&mut self.main_rtv)
        ));
        unsafe {
            back_buffer_ptr.Release();
            back_buffer_ptr = transmute(0usize);
        }
        let mut depth_buffer_desc: d11::D3D11_TEXTURE2D_DESC = unsafe { zeroed() };
        depth_buffer_desc.Width = window.screen_width as mwd::UINT;
        depth_buffer_desc.Height = window.screen_height as mwd::UINT;
        depth_buffer_desc.MipLevels = 1;
        depth_buffer_desc.ArraySize = 1;
        depth_buffer_desc.Format = dxfmt::DXGI_FORMAT_D32_FLOAT;
        depth_buffer_desc.SampleDesc = swap_chain_desc.SampleDesc;
        depth_buffer_desc.Usage = d11::D3D11_USAGE_DEFAULT;
        depth_buffer_desc.BindFlags = d11::D3D11_BIND_DEPTH_STENCIL;
        depth_buffer_desc.CPUAccessFlags = 0;
        depth_buffer_desc.MiscFlags = 0;
        hr_check!(self.device.CreateTexture2D(
            &depth_buffer_desc,
            null_mut(),
            transmute(&mut self.main_dsb)
        ));
        let mut depth_stencil_desc: d11::D3D11_DEPTH_STENCIL_DESC = unsafe { zeroed() };
        depth_stencil_desc.DepthEnable = 1;
        depth_stencil_desc.DepthWriteMask = d11::D3D11_DEPTH_WRITE_MASK_ALL;
        depth_stencil_desc.DepthFunc = d11::D3D11_COMPARISON_LESS;
        depth_stencil_desc.StencilEnable = 1;
        depth_stencil_desc.StencilReadMask = 0xFF;
        depth_stencil_desc.StencilWriteMask = 0xFF;
        depth_stencil_desc.FrontFace.StencilFailOp = d11::D3D11_STENCIL_OP_KEEP;
        depth_stencil_desc.FrontFace.StencilDepthFailOp = d11::D3D11_STENCIL_OP_INCR;
        depth_stencil_desc.FrontFace.StencilPassOp = d11::D3D11_STENCIL_OP_KEEP;
        depth_stencil_desc.FrontFace.StencilFunc = d11::D3D11_COMPARISON_ALWAYS;
        depth_stencil_desc.BackFace.StencilFailOp = d11::D3D11_STENCIL_OP_KEEP;
        depth_stencil_desc.BackFace.StencilDepthFailOp = d11::D3D11_STENCIL_OP_DECR;
        depth_stencil_desc.BackFace.StencilPassOp = d11::D3D11_STENCIL_OP_KEEP;
        depth_stencil_desc.BackFace.StencilFunc = d11::D3D11_COMPARISON_ALWAYS;
        hr_check!(
            self.device
                .CreateDepthStencilState(&depth_stencil_desc, transmute(&mut self.main_dss))
        );
        unsafe {
            self.context.OMSetDepthStencilState(self.main_dss, 1);
        }
        let mut depth_stencil_view_desc: d11::D3D11_DEPTH_STENCIL_VIEW_DESC = unsafe { zeroed() };
        depth_stencil_view_desc.Format = depth_buffer_desc.Format;
        depth_stencil_view_desc.ViewDimension = d11::D3D11_DSV_DIMENSION_TEXTURE2DMS;
        unsafe { depth_stencil_view_desc.u.Texture2D_mut() }.MipSlice = 0;
        hr_check!(self.device.CreateDepthStencilView(
            transmute_copy(self.main_dsb),
            &depth_stencil_view_desc,
            transmute(&mut self.main_dsv)
        ));
        unsafe {
            self.context
                .OMSetRenderTargets(1, transmute(&mut self.main_rtv), self.main_dsv);
        }
        let mut raster_desc: d11::D3D11_RASTERIZER_DESC = unsafe { zeroed() };
        raster_desc.AntialiasedLineEnable = 1;
        raster_desc.CullMode = d11::D3D11_CULL_BACK;
        raster_desc.DepthBiasClamp = 0f32;
        raster_desc.DepthClipEnable = 1;
        raster_desc.FillMode = d11::D3D11_FILL_SOLID;
        raster_desc.FrontCounterClockwise = 1;
        raster_desc.MultisampleEnable = 1;
        raster_desc.ScissorEnable = 0;
        raster_desc.SlopeScaledDepthBias = 0f32;
        hr_check!(
            self.device
                .CreateRasterizerState(&raster_desc, transmute(&mut self.raster))
        );
        unsafe {
            self.context.RSSetState(self.raster);
        }
        self.main_viewport.Width = window.screen_width as f32;
        self.main_viewport.Height = window.screen_height as f32;
        self.main_viewport.MinDepth = 0f32;
        self.main_viewport.MaxDepth = 1f32;
        self.main_viewport.TopLeftX = 0f32;
        self.main_viewport.TopLeftY = 0f32;
        unsafe {
            self.context.RSSetViewports(1, &mut self.main_viewport);
        }
    }
}
