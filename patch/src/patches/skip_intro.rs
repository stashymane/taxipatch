use crate::data::PatchContext;
use crate::Patch;
use game::Global;

inventory::submit! {
    Patch::new("intro skip", initialize)
}

pub fn initialize(ctx: &PatchContext) -> anyhow::Result<()> {
    if !ctx.settings.patches.skip_intro {
        return Ok(());
    }

    unsafe {
        game::hooks::boot_logo_sequence_update.hook({
            move |fun| {
                Global::BOOT_LOGO_FRAME_COUNTER.write(1024);

                fun.call(())
            }
        });
    }
    Ok(())
}
