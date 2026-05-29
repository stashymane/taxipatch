use crate::data::PatchContext;
use crate::game::FrameLimiter;
use retour::static_detour;
use std::mem::transmute;

static_detour! {
    pub static FrameLimiter_Update: unsafe extern "fastcall" fn(*mut FrameLimiter);
}

pub fn initialize(ctx: &PatchContext) -> Result<(), retour::Error> {
    let desired_fov = ctx.settings.game.fov;
    let desired_aspect = ctx.settings.game.aspect_ratio().unwrap();

    unsafe {
        FrameLimiter_Update.initialize(transmute(ctx.offsets.frame_limiter_update), {
            move |frame_limiter_ptr| {
                let frame_limiter: &mut FrameLimiter = &mut *frame_limiter_ptr;

                frame_limiter.adaptive_mode = false;

                FrameLimiter_Update.call(frame_limiter_ptr);
            }
        })?;

        FrameLimiter_Update.enable()?;
    }
    Ok(())
}
