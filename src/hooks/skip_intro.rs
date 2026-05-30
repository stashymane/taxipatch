use crate::data::{PatchContext, CT3};
use retour::static_detour;
use std::mem::transmute;

static_detour! {
    pub static BootLogoSequence_Update: unsafe extern "stdcall" fn();
}

pub fn initialize(ctx: &PatchContext) -> Result<(), retour::Error> {
    unsafe {
        BootLogoSequence_Update.initialize(
            transmute(ctx.offsets[CT3::BootLogoSequenceUpdate]),
            {
                let counter = ctx.pointers.boot_logo_frame_counter;

                move || {
                    counter.write(1024);

                    BootLogoSequence_Update.call();
                }
            },
        )?;

        BootLogoSequence_Update.enable()?;
    }
    Ok(())
}
