use crate::data::{PatchContext, CT3};
use game::BootLogoSequence_UpdateHook;
use std::mem::transmute;

pub fn initialize(ctx: &PatchContext) -> Result<(), retour::Error> {
    unsafe {
        BootLogoSequence_UpdateHook.initialize(
            transmute(ctx.offsets[CT3::BootLogoSequenceUpdate]),
            {
                let counter = ctx.pointers.boot_logo_frame_counter;

                move || {
                    counter.write(1024);

                    BootLogoSequence_UpdateHook.call();
                }
            },
        )?;

        BootLogoSequence_UpdateHook.enable()?;
    }
    Ok(())
}
