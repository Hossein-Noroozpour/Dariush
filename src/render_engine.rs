use winapi::shared::dxgi;
use winapi::um::d3d11 as d11;

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
    shadow_viewport: *mut d11::D3D11_VIEWPORT,
}
