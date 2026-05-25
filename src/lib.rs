pub mod hooks;
pub mod settings;
pub mod windows;

use crate::hooks::offsets::Offsets;
use crate::settings::Settings;
use crate::windows::debug::message_box;
use anyhow::Context;
use std::process::exit;
use ::windows::core::*;
use ::windows::Win32::Foundation::HINSTANCE;
use ::windows::Win32::Foundation::*;
use ::windows::Win32::System::SystemServices::*;

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
pub unsafe extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    reserved: *mut std::ffi::c_void,
) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        if let Err(e) = init() {
            eprintln!("{:?}", e);
            message_box(
                "Failed to apply taxipatch",
                format!(
                    "Error: {:?}\r\n\r\nThe game will now close. Please report this issue in the issue tracker.",
                    e
                ),
            );
            exit(1);
        }
    }
    TRUE
}

fn init() -> anyhow::Result<()> {
    let settings = Settings::load();
    log!("loaded config: {:?}", settings);

    let offsets = Offsets::get_fairlight();

    hooks::resolution::initialize(&offsets, &settings)
        .context("Failed to apply resolution patch")?;

    Ok(())
}
