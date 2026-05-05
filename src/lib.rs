use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::SystemServices::*;
use windows::Win32::UI::WindowsAndMessaging::*;

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    reserved: *mut std::ffi::c_void,
) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        unsafe {
            let _ = MessageBoxA(
                Option::from(HWND::default()),
                s!("hello world"),
                s!("taxipatch"),
                MB_OK | MB_ICONINFORMATION,
            );
        }
    }
    TRUE
}
