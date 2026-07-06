use crate::data::PatchContext;
use crate::patch::Patch;
use game::{CD3DApplication, Camera};

inventory::submit! {
    Patch::new("camera", initialize)
}

pub fn initialize(ctx: &PatchContext) -> anyhow::Result<()> {
    let desired_fov = ctx.settings.game.fov;

    unsafe {
        Camera::update_camera_from_globals.hook(move |_| {
            let cd3d_app = &mut *CD3DApplication::INSTANCE.as_ptr();

            let actual_aspect =
                cd3d_app.initial_window_width as f32 / cd3d_app.initial_window_height as f32;

            let actual_fov = desired_fov.unwrap_or_else(|| fov_from_aspect(actual_aspect));

            Camera::set_perspective(
                Camera::G_CAMERA.read(),
                actual_fov,
                actual_aspect,
                Camera::G_CLIP_NEAR.read(),
                Camera::G_CLIP_FAR.read(),
            )
        });
    }
    Ok(())
}

fn fov_from_aspect(aspect: f32) -> f32 {
    let default_aspect = 4.0_f32 / 3.0_f32;
    let default_fov = 60.0_f32.to_radians();

    let adjusted_fov_rad = 2.0 * ((default_fov / 2.0).tan() * (aspect / default_aspect)).atan();

    adjusted_fov_rad.to_degrees()
}
