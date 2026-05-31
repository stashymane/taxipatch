use crate::data::{PatchContext, CT3};
use crate::Patch;
use game::BootLogoSequence_UpdateHook;
use std::mem::transmute;

inventory::submit! {
    Patch {
        name: "intro skip",
        priority: 0,
        register: initialize
    }
}

pub fn initialize(ctx: &PatchContext) -> anyhow::Result<()> {
    if !ctx.settings.patches.skip_intro {
        return Ok(());
    }

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
