use crate::data::PatchContext;
use crate::patch::Patch;
use crate::patches::camera::apply_renderer_perspective;
use game::{Camera, Effects};

inventory::submit! {
    Patch::new("effects", initialize)
}

/// Effects sometimes get rendered before camera metrics are updated, so they keep receiving stale information.
/// This patch always refreshes camera metrics before rendering effects affected by this bug.
pub fn initialize(ctx: &PatchContext) -> anyhow::Result<()> {
    let desired_fov = ctx.settings.game.fov;

    Effects::draw_destination_borders.hook({
        move |fun| {
            refresh_effect_camera(desired_fov);
            fun.call(());
        }
    });

    Effects::draw_particles.hook({
        move |fun| {
            refresh_effect_camera(desired_fov);
            fun.call(());
        }
    });
    Ok(())
}

fn refresh_effect_camera(desired_fov: Option<f32>) {
    Camera::update();
    apply_renderer_perspective(desired_fov);
}
