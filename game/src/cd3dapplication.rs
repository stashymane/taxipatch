use crate::{CameraProjection, D3DDeviceSettings};
use retour::static_detour;
use static_assertions::assert_eq_size;
use windows::core::HRESULT;
use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::Graphics::Direct3D9::{IDirect3D9, IDirect3DDevice9, D3DPRESENT_PARAMETERS};

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
    pub d3d9: *mut IDirect3D9,
    pub d3d9device: *mut IDirect3DDevice9,

    pub _undefined: [u8; 0x35c - 0xf4], //0xf4 .. 0x35c

    pub window_name: *mut char,
    pub initial_window_width: u32,
    pub initial_window_height: u32,

    _field_0x368: bool,
    _field_0x369: bool,

    pub use_fullscreen_mode: bool,

    _field_0x36b: bool,
}

static_detour! {
    pub static CD3DApplication_InitWindowHook: unsafe extern "thiscall" fn(*mut CD3DApplication, *mut HINSTANCE) -> HRESULT;
    pub static BootLogoSequence_UpdateHook: unsafe extern "stdcall" fn();

    pub static BuildPresentParamsHook: unsafe extern "thiscall" fn(*mut CD3DApplication);
    pub static SetCameraPerspectiveHook: unsafe extern "thiscall" fn(
        *mut CameraProjection,
        f32,
        f32,
        f32,
        f32,
    );
}
