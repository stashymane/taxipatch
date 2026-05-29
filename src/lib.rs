#[cfg(not(target_pointer_width = "32"))]
compile_error!("This patch requires a 32-bit target because the game is 32-bit.");

pub mod data;
pub mod game;
pub mod hooks;
pub mod windows;

use crate::data::{ExecutableType, PatchContext, Settings};
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
                format!("{:?}\r\n\r\nThe game will now close.", e),
            );
            exit(1);
        }
    }
    TRUE
}

fn init() -> anyhow::Result<()> {
    let exe_type = ExecutableType::load()?;
    let settings = Settings::load().context("Failed to load settings")?;

    match exe_type {
        ExecutableType::Config => {
            log!("No patches available for config - skipping...");
        }
        ExecutableType::Fairlight(offsets) => {
            let ctx = PatchContext::from(offsets, settings)?;
            log!("loaded context: {:?}", ctx);

            hooks::resolution::initialize(&ctx).context("Failed to apply resolution patch")?;
            if ctx.settings.patches.skip_intro {
                hooks::skip_intro::initialize(&ctx).context("Failed to apply intro skip patch")?;
            }
        }
    };

    Ok(())
}
