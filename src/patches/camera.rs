use crate::data::{PatchContext, CT3};
use crate::patch::Patch;
use game::SetCameraPerspectiveHook;
use std::mem::transmute;

inventory::submit! {
    Patch {
        name: "camera",
        priority: 0,
        register: initialize
    }
}

pub fn initialize(ctx: &PatchContext) -> anyhow::Result<()> {
    let desired_fov = ctx.settings.game.fov;
    let desired_aspect = ctx.settings.game.aspect_ratio().unwrap();

    unsafe {
        SetCameraPerspectiveHook.initialize(
            transmute(ctx.offsets[CT3::SetCameraPerspective]),
            {
                let stage = ctx.pointers.game_stage;
                let substage = ctx.pointers.game_substage;

                move |camera, fov, aspect, near_clip, far_clip| {
                    let (fov, aspect) = if stage.read() == 1 && substage.read() == 3 {
                        (desired_fov, desired_aspect)
                    } else {
                        (fov, aspect)
                    };

                    return SetCameraPerspectiveHook.call(camera, fov, aspect, near_clip, far_clip);
                }
            },
        )?;

        SetCameraPerspectiveHook.enable()?;
    }
    Ok(())
}
