use retour_utils::hook_impl;
use std::ffi::c_void;
use windows::core::PCSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND};
use windows::Win32::UI::WindowsAndMessaging::{HCURSOR, HMENU, WINDOW_EX_STYLE, WINDOW_STYLE};

pub struct User32;

#[hook_impl("user32.dll")]
impl User32 {
    #[hook(pub unsafe extern "system" User32_CreateWindowExA, symbol = "CreateWindowExA", chain)]
    pub fn create_window_ex_a(
        dw_ex_style: WINDOW_EX_STYLE,
        lp_class_name: PCSTR,
        lp_window_name: PCSTR,
        dw_style: WINDOW_STYLE,
        x: i32,
        y: i32,
        n_width: i32,
        n_height: i32,
        h_wnd_parent: HWND,
        h_menu: HMENU,
        h_instance: HINSTANCE,
        lp_param: *const c_void,
    ) -> HWND {
        unsafe {
            User32_CreateWindowExA.call(
                dw_ex_style,
                lp_class_name,
                lp_window_name,
                dw_style,
                x,
                y,
                n_width,
                n_height,
                h_wnd_parent,
                h_menu,
                h_instance,
                lp_param,
            )
        }
    }

    #[hook(unsafe extern "system" User32_SetCursor, symbol = "SetCursor")]
    pub fn set_cursor(cursor: HCURSOR) -> HCURSOR {
        unsafe { User32_SetCursor.call(cursor) }
    }
}
