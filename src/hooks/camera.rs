use crate::data::PatchContext;
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
        SetCameraPerspective.initialize(transmute(ctx.offsets.set_camera_perspective), {
            let stage_addr = ctx.offsets.globals.game_stage;
            let substage_addr = ctx.offsets.globals.game_substage;

            move |camera, fov, aspect, near_clip, far_clip| {
                let stage = stage_addr as *mut u32;
                let substage = substage_addr as *mut u32;

                let (fov, aspect) = if *stage == 1 && *substage == 3 {
                    (desired_fov, desired_aspect)
                } else {
                    (fov, aspect)
                };

                return SetCameraPerspective.call(camera, fov, desired_aspect, near_clip, far_clip);
            }
        })?;

        SetCameraPerspective.enable()?;
    }
    Ok(())
}
