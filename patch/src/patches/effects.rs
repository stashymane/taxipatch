use crate::data::PatchContext;
use crate::patch::Patch;
use crate::patches::camera::{apply_renderer_perspective, is_in_race};
use game::{Camera, Effects};
use std::sync::atomic::{AtomicBool, Ordering};

inventory::submit! {
    Patch::new("effects", initialize).priority(-1)
}

/// When set, destination/particle draws are skipped at their early `RenderFrame` sites
/// and flushed after `UpdateCameraFromGlobals`.
static DEFER_EFFECT_DRAWS: AtomicBool = AtomicBool::new(false);

/// Allows the deferred flush to run the real draw hooks without re-deferring.
static FLUSHING_EFFECT_DRAWS: AtomicBool = AtomicBool::new(false);

/// Destination borders and particles draw in `RenderFrame` before the FOV pass, while
/// chase follow targets are only refreshed at the end of `InGameState`. Defer those
/// draws until after `UpdateCameraFromGlobals`, then rebuild the view once so effects
/// use this frame's follow targets and the patched FOV.
pub fn initialize(ctx: &PatchContext) -> anyhow::Result<()> {
    let desired_fov = ctx.settings.game.fov;

    Effects::draw_destination_borders.hook(|fun| {
        if FLUSHING_EFFECT_DRAWS.load(Ordering::Relaxed) {
            return fun.call(());
        }
        if is_in_race() {
            DEFER_EFFECT_DRAWS.store(true, Ordering::Relaxed);
            return;
        }
        fun.call(());
    });

    Effects::draw_particles.hook(|fun| {
        if FLUSHING_EFFECT_DRAWS.load(Ordering::Relaxed) {
            return fun.call(());
        }
        if DEFER_EFFECT_DRAWS.load(Ordering::Relaxed) {
            return;
        }
        fun.call(());
    });

    Camera::update_camera_from_globals.hook(move |original| {
        original.call(());

        if !DEFER_EFFECT_DRAWS.swap(false, Ordering::Relaxed) {
            return;
        }

        Camera::update.call();
        apply_renderer_perspective(desired_fov);

        FLUSHING_EFFECT_DRAWS.store(true, Ordering::Relaxed);
        Effects::draw_destination_borders.call();
        Effects::draw_particles.call();
        FLUSHING_EFFECT_DRAWS.store(false, Ordering::Relaxed);
    });

    Ok(())
}
