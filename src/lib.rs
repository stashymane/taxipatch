pub mod settings;
pub mod windows;

use crate::settings::Settings;
use ::windows::core::*;
use ::windows::Win32::Foundation::HINSTANCE;
use ::windows::Win32::Foundation::*;
use ::windows::Win32::System::SystemServices::*;

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    reserved: *mut std::ffi::c_void,
) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        init();
    }
    TRUE
}

fn init() {
    let settings = Settings::load();
    log!("loaded config: {:?}", settings);
}
