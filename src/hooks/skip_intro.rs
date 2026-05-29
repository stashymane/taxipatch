use crate::data::PatchContext;
use retour::static_detour;
use std::mem::transmute;

static_detour! {
    pub static BootLogoSequence_Update: unsafe extern "stdcall" fn();
}

pub fn initialize(ctx: &PatchContext) -> Result<(), retour::Error> {
    unsafe {
        BootLogoSequence_Update.initialize(transmute(ctx.offsets.boot_logo_sequence_update), {
            let counter_offset = ctx.offsets.boot_logo_frame_counter;
            move || {
                let counter = counter_offset as *mut i32;
                *counter = 1024;

                BootLogoSequence_Update.call();
            }
        })?;

        BootLogoSequence_Update.enable()?;
    }
    Ok(())
}
