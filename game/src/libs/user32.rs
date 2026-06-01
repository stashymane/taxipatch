// USER32.DLL hooks

use retour::static_detour;
use std::ffi::c_void;
use windows::core::PCSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::UI::WindowsAndMessaging::{HCURSOR, HMENU, WINDOW_EX_STYLE, WINDOW_STYLE};

static_detour! {
    pub static CreateWindowExAHook: unsafe extern "system" fn(
        WINDOW_EX_STYLE,
        PCSTR,
        PCSTR,
        WINDOW_STYLE,
        i32,
        i32,
        i32,
        i32,
        HWND,
        HMENU,
        HINSTANCE,
        *const c_void,
    ) -> HWND;
}

pub type SetCursor = unsafe extern "system" fn(HCURSOR) -> HCURSOR;
