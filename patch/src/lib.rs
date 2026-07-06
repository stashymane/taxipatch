#![feature(fn_traits)]

#[cfg(any(not(target_os = "windows"), not(target_pointer_width = "32")))]
compile_error!("This patch can only be compiled for 32-bit Windows.");

pub mod data;
pub mod patch;
pub mod patches;
pub mod windows;

use crate::data::{ExecutableType, PatchContext, Settings};
use crate::patch::Patch;
use crate::windows::debug::message_box;
use anyhow::Context;
use game::audio::{CAudioStream, CStreamHandler};
use game::libs::user32::User32;
use game::{CD3DApplication, Camera, FrameLimiter, Global};
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
            Ok(())
        }
        ExecutableType::Xplosiv | ExecutableType::Fairlight => {
            let ctx = PatchContext::from(settings)?;
            log!("Loaded context: {:?}", ctx);

            let mut patches: Vec<_> = inventory::iter::<Patch>.into_iter().collect();
            patches.sort_by_key(|it| -it.priority);

            for patch in patches {
                if !patch.enabled {
                    log!("Patch {} disabled, skipping...", patch.name);
                    continue;
                }
                patch
                    .register
                    .call_once((&ctx,))
                    .with_context(|| format!("Applying {} patch", patch.name))?;
            }

            log!("Applying hooks...");

            Global::init_detours()?;
            User32::init_detours()?;
            CD3DApplication::init_detours()?;
            CAudioStream::init_detours()?;
            CStreamHandler::init_detours()?;
            Camera::init_detours()?;
            FrameLimiter::init_detours()?;
            game::hooks::init_detours()?;
            // game::ct3config::init_detours()?;

            Ok(())
        }
    }
}
