use winapi::shared::{
    dxgi,
    minwindef as mwd,
};
use winapi::um::d3d11 as d11;
use std::ptr::null_mut;

pub struct RenderEngine {
    swapchain: *mut dxgi::IDXGISwapChain,
    device: *mut d11::ID3D11Device,
    context: *mut d11::ID3D11DeviceContext,
    main_rtv: *mut d11::ID3D11RenderTargetView,
    main_dsb: *mut d11::ID3D11Texture2D,
    main_dss: *mut d11::ID3D11DepthStencilState,
    main_dsv: *mut d11::ID3D11DepthStencilView,
    main_viewport: d11::D3D11_VIEWPORT,
    raster: *mut d11::ID3D11RasterizerState,
    shadow_rtv: *mut d11::ID3D11RenderTargetView,
    shadow_dsb: *mut d11::ID3D11Texture2D,
    shadow_dsv: *mut d11::ID3D11DepthStencilView,
    shadow_viewport: d11::D3D11_VIEWPORT,
}

impl RenderEngine {
    pub fn new() -> Self {
        RenderEngine {
            swapchain: null_mut(),
            device: null_mut(),
            context: null_mut(),
            main_rtv: null_mut(),
            main_dsb: null_mut(),
            main_dss: null_mut(),
            main_dsv: null_mut(),
            main_viewport: d11::D3D11_VIEWPORT {
                TopLeftX: 0f32,
                TopLeftY: 0f32,
                Width: 0f32,
                Height: 0f32,
                MinDepth: 0f32,
                MaxDepth: 0f32,
            },
            raster: null_mut(),
            shadow_rtv: null_mut(),
            shadow_dsb: null_mut(),
            shadow_dsv: null_mut(),
            shadow_viewport: d11::D3D11_VIEWPORT {
                TopLeftX: 0f32,
                TopLeftY: 0f32,
                Width: 0f32,
                Height: 0f32,
                MinDepth: 0f32,
                MaxDepth: 0f32,
            },
        }
    }

    pub fn initialize(&mut self) {
        let mut factory: *mut dxgi::IDXGIFactory = null_mut();
        let mut adapter: *mut dxgi::IDXGIAdapter = null_mut();
        let mut adapter_output: *mut dxgi::IDXGIOutput = null_mut();
        let mut numerator: mwd::UINT = 0;
        let mut denominator: mwd::UINT = 0;
        hr_check!(dxgi::CreateDXGIFactory(dxi::IDXGIFactory::uuidof(), &mut factory));
        let mut adapter_index: mwd::UINT = 0;
        loop {
            if factory->EnumAdapters(adapter_index, &adapter) == dxgi::DXGI_ERROR_NOT_FOUND {
                break;
            }
            DXGI_ADAPTER_DESC adapter_desc;
            GXHRCHK(adapter->GetDesc(&adapter_desc));
            graphic_memory_size = (unsigned int)adapter_desc.DedicatedVideoMemory;
            char video_card_description[128];
            size_t strlen;
            if (wcstombs_s(&strlen, video_card_description, 128, adapter_desc.Description, 128) != 0) {
                UNEXPECTED;
            }
            for (size_t i = strlen; i < 128; ++i)
                video_card_description[i] = 0;
            GXLOGD("Video Card Description: " << video_card_description);
            for (UINT adpout_index = 0;
                adapter->EnumOutputs(adpout_index, &adapter_output) != DXGI_ERROR_NOT_FOUND;
                ++adpout_index, adapter_output->Release(), adapter_output = nullptr) {
                UINT num_modes = 0;
                if (FAILED(adapter_output->GetDisplayModeList(DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_ENUM_MODES_INTERLACED, &num_modes, NULL))) {
                    continue;
                }
                if (num_modes == 0) {
                    continue;
                }
                std::vector<DXGI_MODE_DESC> display_mode_list(num_modes);
                if (FAILED(adapter_output->GetDisplayModeList(DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_ENUM_MODES_INTERLACED, &num_modes, &(display_mode_list[0])))) {
                    continue;
                }
                for (UINT i = 0; i < num_modes; i++) {
                    if (display_mode_list[i].Width == sysapp->get_width()) {
                        if (display_mode_list[i].Height == sysapp->get_height()) {
                            numerator = display_mode_list[i].RefreshRate.Numerator;
                            denominator = display_mode_list[i].RefreshRate.Denominator;
                        }
                    }
                }
                goto adapter_found_label;
            }
            adapter_index += 1;
            adapter.Release();
            adapter = null_mut();
        }
adapter_found_label:
    if (adapter_output != nullptr)
        adapter_output->Release();
    if (adapter != nullptr)
        adapter->Release();
    if (factory != nullptr)
        factory->Release();
    DXGI_SWAP_CHAIN_DESC swap_chain_desc;
    GXSETZ(swap_chain_desc);
    swap_chain_desc.BufferCount = 1;
    swap_chain_desc.BufferDesc.Width = sysapp->get_width();
    swap_chain_desc.BufferDesc.Height = sysapp->get_height();
    swap_chain_desc.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    swap_chain_desc.BufferDesc.RefreshRate.Numerator = numerator;
    swap_chain_desc.BufferDesc.RefreshRate.Denominator = denominator;
    swap_chain_desc.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
    swap_chain_desc.OutputWindow = sysapp->get_window();
    swap_chain_desc.SampleDesc.Count = 1;
#ifdef GEAROENIX_FULLSCREEN
    swap_chain_desc.Windowed = false;
#else
    swap_chain_desc.Windowed = true;
#endif
    swap_chain_desc.BufferDesc.ScanlineOrdering = DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED;
    swap_chain_desc.BufferDesc.Scaling = DXGI_MODE_SCALING_UNSPECIFIED;
    swap_chain_desc.SwapEffect = DXGI_SWAP_EFFECT_DISCARD;
    swap_chain_desc.Flags = 0;

    UINT device_flag =
#ifdef DEBUG_MODE
        D3D11_CREATE_DEVICE_DEBUG;
#else
        D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT;
#endif
    const D3D_FEATURE_LEVEL feature_level = D3D_FEATURE_LEVEL_11_0;
    D3D_DRIVER_TYPE driver_type = D3D_DRIVER_TYPE_HARDWARE;
    if (FAILED(D3D11CreateDevice(
            nullptr, driver_type, nullptr, device_flag,
            &feature_level, 1, D3D11_SDK_VERSION,
            &device, nullptr, nullptr))) {
#ifdef DEBUG_MODE
        driver_type = D3D_DRIVER_TYPE_REFERENCE;
        GXHRCHK(D3D11CreateDevice(
            nullptr, driver_type, nullptr, device_flag,
            &feature_level, 1, D3D11_SDK_VERSION,
            &device, nullptr, nullptr));
#endif
    }
    for (unsigned int i = D3D11_MAX_MULTISAMPLE_SAMPLE_COUNT; i > 0; --i) {
        swap_chain_desc.SampleDesc.Count = i;
        UINT sample_quality;
        device->CheckMultisampleQualityLevels(
            swap_chain_desc.BufferDesc.Format,
            i, &sample_quality);
        if (sample_quality > 0) {
            --sample_quality;
            swap_chain_desc.SampleDesc.Quality = sample_quality;
            swap_chain_desc.SampleDesc.Count = i;
            GXLOGD("count " << i << ",  quality: " << sample_quality);
            break;
        }
    }
    device->Release();
    device = nullptr;
    GXHRCHK(D3D11CreateDeviceAndSwapChain(
        nullptr, driver_type, nullptr, device_flag,
        &feature_level, 1, D3D11_SDK_VERSION, &swap_chain_desc, &swapchain,
        &device, nullptr, &context));
    ID3D11Texture2D* back_buffer_ptr;
    GXHRCHK(swapchain->GetBuffer(0, __uuidof(ID3D11Texture2D), (LPVOID*)&back_buffer_ptr));
    GXHRCHK(device->CreateRenderTargetView(back_buffer_ptr, NULL, &main_rtv));
    back_buffer_ptr->Release();
    back_buffer_ptr = nullptr;
    D3D11_TEXTURE2D_DESC depth_buffer_desc;
    GXSETZ(depth_buffer_desc);
    depth_buffer_desc.Width = sysapp->get_width();
    depth_buffer_desc.Height = sysapp->get_height();
    depth_buffer_desc.MipLevels = 1;
    depth_buffer_desc.ArraySize = 1;
    depth_buffer_desc.Format = DXGI_FORMAT_D32_FLOAT;
    depth_buffer_desc.SampleDesc = swap_chain_desc.SampleDesc;
    depth_buffer_desc.Usage = D3D11_USAGE_DEFAULT;
    depth_buffer_desc.BindFlags = D3D11_BIND_DEPTH_STENCIL;
    depth_buffer_desc.CPUAccessFlags = 0;
    depth_buffer_desc.MiscFlags = 0;
    GXHRCHK(device->CreateTexture2D(&depth_buffer_desc, NULL, &main_dsb));
    D3D11_DEPTH_STENCIL_DESC depth_stencil_desc;
    GXSETZ(depth_stencil_desc);
    depth_stencil_desc.DepthEnable = true;
    depth_stencil_desc.DepthWriteMask = D3D11_DEPTH_WRITE_MASK_ALL;
    depth_stencil_desc.DepthFunc = D3D11_COMPARISON_LESS;
    depth_stencil_desc.StencilEnable = true;
    depth_stencil_desc.StencilReadMask = 0xFF;
    depth_stencil_desc.StencilWriteMask = 0xFF;
    depth_stencil_desc.FrontFace.StencilFailOp = D3D11_STENCIL_OP_KEEP;
    depth_stencil_desc.FrontFace.StencilDepthFailOp = D3D11_STENCIL_OP_INCR;
    depth_stencil_desc.FrontFace.StencilPassOp = D3D11_STENCIL_OP_KEEP;
    depth_stencil_desc.FrontFace.StencilFunc = D3D11_COMPARISON_ALWAYS;
    depth_stencil_desc.BackFace.StencilFailOp = D3D11_STENCIL_OP_KEEP;
    depth_stencil_desc.BackFace.StencilDepthFailOp = D3D11_STENCIL_OP_DECR;
    depth_stencil_desc.BackFace.StencilPassOp = D3D11_STENCIL_OP_KEEP;
    depth_stencil_desc.BackFace.StencilFunc = D3D11_COMPARISON_ALWAYS;
    GXHRCHK(device->CreateDepthStencilState(&depth_stencil_desc, &main_dss));
    context->OMSetDepthStencilState(main_dss, 1);
    D3D11_DEPTH_STENCIL_VIEW_DESC depth_stencil_view_desc;
    GXSETZ(depth_stencil_view_desc);
    depth_stencil_view_desc.Format = depth_buffer_desc.Format;
    depth_stencil_view_desc.ViewDimension = D3D11_DSV_DIMENSION_TEXTURE2DMS;
    depth_stencil_view_desc.Texture2D.MipSlice = 0;
    GXHRCHK(device->CreateDepthStencilView(main_dsb, &depth_stencil_view_desc, &main_dsv));
    context->OMSetRenderTargets(1, &main_rtv, main_dsv);
    D3D11_RASTERIZER_DESC raster_desc;
    GXSETZ(raster_desc);
    raster_desc.AntialiasedLineEnable = true;
    raster_desc.CullMode = D3D11_CULL_BACK;
    raster_desc.DepthBiasClamp = 0.0f;
    raster_desc.DepthClipEnable = true;
    raster_desc.FillMode = D3D11_FILL_SOLID;
    raster_desc.FrontCounterClockwise = true;
    raster_desc.MultisampleEnable = true;
    raster_desc.ScissorEnable = false;
    raster_desc.SlopeScaledDepthBias = 0.0f;
    GXHRCHK(device->CreateRasterizerState(&raster_desc, &raster));
    context->RSSetState(raster);
    main_viewport.Width = (float)sysapp->get_width();
    main_viewport.Height = (float)sysapp->get_height();
    main_viewport.MinDepth = 0.0f;
    main_viewport.MaxDepth = 1.0f;
    main_viewport.TopLeftX = 0.0f;
    main_viewport.TopLeftY = 0.0f;
    context->RSSetViewports(1, &main_viewport);
    sampler = new texture::Sampler(this);
    pipmgr = new render::pipeline::Manager(this);
    initial_shadow();
    }
}
