use crate::D3DDeviceSettings;
use retour_utils::hook_impl;
use static_assertions::assert_eq_size;
use windows::core::HRESULT;
use windows::Win32::Foundation::{HINSTANCE, HWND, LRESULT, WPARAM};
use windows::Win32::Graphics::Direct3D9::{IDirect3D9, IDirect3DDevice9, D3DPRESENT_PARAMETERS};
use windows::Win32::UI::WindowsAndMessaging::MINMAXINFO;

assert_eq_size!(CD3DApplication, [u8; 0x36c]);

#[repr(C)]
#[derive(Debug)]
pub struct CD3DApplication {
    pub vtable: usize,

    _device_enumerator: [u8; 0x2c],

    pub is_windowed: bool,
    _pad_0x31: [u8; 0x03],

    pub windowed_settings: D3DDeviceSettings,
    pub windowed_height: u32,
    pub windowed_width: u32,

    pub fullscreen_settings: D3DDeviceSettings,
    pub use_fallback_d3d_mode: bool,

    _mbr_0x9d: [u8; 4],
    pub ignore_window_size_change: u8,
    _mbr_0xa2: [u8; 6],

    pub present_parameters: D3DPRESENT_PARAMETERS,

    pub window_handle: HWND,
    pub _field_0xe4: usize,
    pub _field_0xe8: usize,
    pub d3d9: IDirect3D9,
    pub d3d9device: IDirect3DDevice9,

    pub _undefined: [u8; 0x35c - 0xf4], //0xf4 .. 0x35c

    pub window_name: *mut char,
    pub initial_window_width: u32,
    pub initial_window_height: u32,

    _field_0x368: bool,
    _field_0x369: bool,

    pub use_fullscreen_mode: bool,

    _field_0x36b: bool,
}

#[hook_impl]
impl CD3DApplication {
    #[ptr(offset = 0x00314f70)]
    pub const INSTANCE: CD3DApplication = null_mut();

    #[hook(pub unsafe extern "thiscall" CD3DApplication_InitWindow, offset = 0x00028da0, chain)]
    pub fn init_window(app_ptr: *mut CD3DApplication, hinstance: *mut HINSTANCE) -> HRESULT {
        unsafe { CD3DApplication_InitWindow.call(app_ptr, hinstance) }
    }

    #[hook(pub unsafe extern "thiscall" CD3DApplication_WndProcDispatcher, offset = 0x00029010, chain)]
    pub fn wnd_proc_dispatcher(
        this: *mut CD3DApplication,
        hwnd: HWND,
        msg: u32,
        w_param: WPARAM,
        l_param: *mut MINMAXINFO,
    ) -> LRESULT {
        unsafe { CD3DApplication_WndProcDispatcher.call(this, hwnd, msg, w_param, l_param) }
    }

    #[hook(pub unsafe extern "thiscall" CD3DApplication_BuildPresentParams, offset = 0x000283d0, chain)]
    pub fn build_present_params(app_ptr: *mut CD3DApplication) {
        unsafe { CD3DApplication_BuildPresentParams.call(app_ptr) }
    }
}
