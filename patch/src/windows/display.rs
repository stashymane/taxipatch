use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

#[derive(Debug, Copy, Clone)]
pub struct DisplayInfo {
    pub width: u32,
    pub height: u32,
}

pub fn get_display_info() -> DisplayInfo {
    unsafe {
        DisplayInfo {
            width: GetSystemMetrics(SM_CXSCREEN) as u32,
            height: GetSystemMetrics(SM_CYSCREEN) as u32,
        }
    }
}
