use static_assertions::assert_eq_size;
use windows::Win32::Graphics::Direct3D9::{
    D3DADAPTER_IDENTIFIER9, D3DCAPS9, D3DDEVTYPE, D3DDISPLAYMODE, D3DFORMAT, D3DMULTISAMPLE_TYPE,
};

type DynArray = [u8; 0x30]; //TODO define dynamic list interface?

#[repr(C)]
#[derive(Debug)]
pub struct D3DDeviceSettings {
    pub adapter_info: *mut D3DEnumAdapterInfo,
    pub device_info: *mut D3DEnumDeviceInfo,
    pub device_settings_combo: *mut D3DEnumDeviceSettingsCombo,
    pub display_mode: D3DDISPLAYMODE,
    pub depth_stencil_format: D3DFORMAT,
    pub multisample_type: D3DMULTISAMPLE_TYPE,
    pub multisample_quality: i32,
    pub vertex_processing_type: i32,
    pub present_interval: u32,
}

assert_eq_size!(D3DDeviceSettings, [u8; 0x30]);

#[repr(C)]
pub struct D3DEnumAdapterInfo {
    pub adapter_ordinal: u32,
    pub identifier: D3DADAPTER_IDENTIFIER9,
    _display_mode_list: *mut DynArray,
    _device_info_list: *mut DynArray,
}

assert_eq_size!(D3DEnumAdapterInfo, [u8; 0x458]);

#[repr(C)]
#[derive(Debug)]
pub struct D3DEnumDeviceInfo {
    pub adapter_ordinal: u32,
    pub device_type: D3DDEVTYPE,
    pub caps: D3DCAPS9,
    _device_settings_combo_list: *mut DynArray,
}

assert_eq_size!(D3DEnumDeviceInfo, [u8; 0x13c]);

#[repr(C)]
#[derive(Debug)]
pub struct D3DEnumDeviceSettingsCombo {
    pub adapter_ordinal: u32,
    pub device_type: D3DDEVTYPE,
    pub adapter_format: D3DFORMAT,
    pub back_buffer_format: D3DFORMAT,
    pub windowed: bool,
    depth_stencil_formts: *mut DynArray,
    multisample_types: *mut DynArray,
    multisample_qualities: *mut DynArray,
    _depth_stencil_multi_sample_conflicts: *mut DynArray,
    vertex_processing_types: *mut DynArray,
    present_intervals: *mut DynArray,
}

assert_eq_size!(D3DEnumDeviceSettingsCombo, [u8; 0x2c]);
