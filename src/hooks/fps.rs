use crate::data::PatchContext;
use crate::game::FrameLimiter;
use crate::log;
use retour::static_detour;
use std::mem::transmute;

static_detour! {
    pub static FrameLimiter_Update: unsafe extern "fastcall" fn(*mut FrameLimiter);
}

pub fn initialize(ctx: &PatchContext) -> Result<(), retour::Error> {
    unsafe {
        FrameLimiter_Update.initialize(transmute(ctx.offsets.frame_limiter_update), {
            move |frame_limiter_ptr| {
                let frame_limiter: &mut FrameLimiter = &mut *frame_limiter_ptr;

                log!(
                    "framelimiter | enabled: {}, adaptive: {}, firstFrame: {}",
                    frame_limiter.limiter_enabled,
                    frame_limiter.adaptive_mode,
                    frame_limiter.first_frame
                );

                frame_limiter.adaptive_mode = false;

                FrameLimiter_Update.call(frame_limiter_ptr);
            }
        })?;

        FrameLimiter_Update.enable()?;
    }
    Ok(())
}
