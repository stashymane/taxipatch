use crate::data::PatchContext;
use crate::patch::Patch;
use game::SetCameraPerspectiveHook;

inventory::submit! {
    Patch {
        name: "camera",
        priority: 0,
        register: initialize
    }
}

pub fn initialize(ctx: &PatchContext) -> anyhow::Result<()> {
    let desired_fov = ctx.settings.game.fov.unwrap_or_else(|| 60.0);

    unsafe {
        SetCameraPerspectiveHook.wrap({
            let stage = ctx.pointers.game_stage;
            let substage = ctx.pointers.game_substage;
            let cd3d_app = ctx.pointers.cd3d_app;

            move |fun, camera, fov, aspect, near_clip, far_clip| {
                let (fov, aspect) = if stage.read() == 1 && substage.read() == 3 {
                    let cd3d_app = cd3d_app.as_ref();

                    let actual_aspect = cd3d_app.initial_window_width as f32
                        / cd3d_app.initial_window_height as f32;
                    (desired_fov, actual_aspect)
                } else {
                    (fov, aspect)
                };

                return fun.call(camera, fov, aspect, near_clip, far_clip);
            }
        })?;
    }
    Ok(())
}
