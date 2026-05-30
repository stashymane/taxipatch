use crate::data::{PatchContext, CT3};
use retour::static_detour;
use std::mem::transmute;

#[repr(C)]
struct CameraProjection {
    _private: [u8; 0],
}

static_detour! {
    pub static SetCameraPerspective: unsafe extern "thiscall" fn(
        *mut CameraProjection,
        f32,
        f32,
        f32,
        f32,
    );
}

pub fn initialize(ctx: &PatchContext) -> Result<(), retour::Error> {
    let desired_fov = ctx.settings.game.fov;
    let desired_aspect = ctx.settings.game.aspect_ratio().unwrap();

    unsafe {
        SetCameraPerspective.initialize(transmute(ctx.offsets[CT3::SetCameraPerspective]), {
            let stage = ctx.pointers.game_stage;
            let substage = ctx.pointers.game_substage;

            move |camera, fov, aspect, near_clip, far_clip| {
                let (fov, aspect) = if stage.read() == 1 && substage.read() == 3 {
                    (desired_fov, desired_aspect)
                } else {
                    (fov, aspect)
                };

                return SetCameraPerspective.call(camera, fov, aspect, near_clip, far_clip);
            }
        })?;

        SetCameraPerspective.enable()?;
    }
    Ok(())
}
