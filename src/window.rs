use winapi::shared::minwindef::{
    HINSTANCE,
    LPARAM,
    LRESULT,
    UINT,
    WPARAM,
};
use winapi::shared::windef::{
    HBRUSH,
    HWND,
};
use winapi::um::winuser::{
    CS_HREDRAW,
    CS_OWNDC,
    CS_VREDRAW,
    WNDCLASSEXW,
};
use winapi::um::wingdi::{
    BLACK_BRUSH,
    GetStockObject,
};
use winapi::um::libloaderapi::GetModuleHandleW;
use std::mem::{
    size_of,
    zeroed
};

struct Window {
    screen_width: u32,
    screen_height: u32,
    instance: HINSTANCE,
    window: HWND,
}

extern "stdcall" fn wnd_process(
    window: HWND, message: UINT, param1: WPARAM, param2: LPARAM) -> LRESULT {
}

impl Window {
    pub fn new() -> Self {
        let mut wc: WNDCLASSEXW = unsafe { zeroed() };
        let instance = GetModuleHandleW(0) as HINSTANCE;
        wc.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC;
        wc.lpfnWndProc = wnd_process;
        wc.hInstance = instance;
        wc.hbrBackground = GetStockObject(BLACK_BRUSH) as HBRUSH;
        wc.lpszClassName = APPLICATION_NAME;
        wc.cbSize = size_of<WNDCLASSEXW>();
        RegisterClassEx(&wc);
        screen_width = GetSystemMetrics(SM_CXSCREEN);
        screen_height = GetSystemMetrics(SM_CYSCREEN);
        #ifdef GEAROENIX_FULLSCREEN
        DEVMODE screen_settings;
        GXSETZ(screen_settings);
        screen_settings.dmSize = sizeof(screen_settings);
        screen_settings.dmPelsWidth = (unsigned long)screen_width;
        screen_settings.dmPelsHeight = (unsigned long)screen_height;
        screen_settings.dmBitsPerPel = 32;
        screen_settings.dmFields = DM_BITSPERPEL | DM_PELSWIDTH | DM_PELSHEIGHT;
        ChangeDisplaySettings(&screen_settings, CDS_FULLSCREEN);
        pos_x = pos_y = 0;
        #else
        screen_width = DEFAULT_WINDOW_WIDTH;
        screen_height = DEFAULT_WINDOW_HEIGHT;
        pos_x = (GetSystemMetrics(SM_CXSCREEN) - screen_width) / 2;
        pos_y = (GetSystemMetrics(SM_CYSCREEN) - screen_height) / 2;
        #endif
        window = CreateWindowEx(WS_EX_APPWINDOW, APPLICATION_NAME, APPLICATION_NAME,
                                WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_POPUP,
                                pos_x, pos_y, screen_width, screen_height, NULL, NULL, instance, this);
        ShowWindow(window, SW_SHOW);
        SetForegroundWindow(window);
        SetFocus(window);
        UpdateWindow(window);
        #ifdef GEAROENIX_NO_CURSOR
        ShowCursor(false);
        #endif
        while (!window_is_up) {
            MSG msg;
            GetMessage(&msg, NULL, 0, 0);
            TranslateMessage(&msg);
            DispatchMessage(&msg);
        }
        #ifdef USE_VULKAN
        if (vulkan::Engine::is_supported())
        render_engine = new vulkan::Engine(this);
        else
        #endif
        #ifdef USE_DIRECTX12
        if (dx12::Engine::is_supported())
        render_engine = new dx12::Engine(this);
        else
        #endif
        #ifdef USE_DIRECTX11
        if (dx11::Engine::is_supported())
        render_engine = new dx11::Engine(this);
        else
        #endif
        #ifdef USE_OPENGL_41
        if (gl41::Engine::is_supported())
        render_engine = new gl41::Engine(this);
        else
        #endif
        #ifdef USE_OPENGL_33
        if (gl33::Engine::is_supported())
        render_engine = new gl33::Engine(this);
        else
        #endif
            {
                GXLOGF("No suitable API found.");
            }
        astmgr = new core::asset::Manager(this, "data.gx3d");
        astmgr->initialize();
        Window {

        }
    }
}