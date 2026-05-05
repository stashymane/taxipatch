use std::sync::OnceLock;
use windows::Win32::Graphics::Gdi::{EnumDisplaySettingsW, DEVMODEW, ENUM_CURRENT_SETTINGS};

pub struct DisplayInfo {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}

pub fn get_display_info() -> &'static DisplayInfo {
    static INFO: OnceLock<DisplayInfo> = OnceLock::new();
    INFO.get_or_init(|| {
        let mut dev_mode = DEVMODEW::default();
        dev_mode.dmSize = size_of::<DEVMODEW>() as u16;

        unsafe {
            if EnumDisplaySettingsW(None, ENUM_CURRENT_SETTINGS, &mut dev_mode).as_bool() {
                DisplayInfo {
                    width: dev_mode.dmPelsWidth,
                    height: dev_mode.dmPelsHeight,
                    refresh_rate: dev_mode.dmDisplayFrequency,
                }
            } else {
                DisplayInfo {
                    width: 1920,
                    height: 1080,
                    refresh_rate: 60,
                }
            }
        }
    })
}
