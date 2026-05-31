use crate::data::PatchContext;
use crate::Patch;
use game::BootLogoSequence_UpdateHook;

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
        BootLogoSequence_UpdateHook.wrap({
            let counter = ctx.pointers.boot_logo_frame_counter;

            move |fun| {
                counter.write(1024);

                fun.call()
            }
        })?;
    }
    Ok(())
}
