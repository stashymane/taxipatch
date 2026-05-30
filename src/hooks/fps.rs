use crate::data::{PatchContext, CT3};
use crate::game::FrameLimiter;
use retour::static_detour;
use std::mem::transmute;

static_detour! {
    pub static FrameLimiter_Update: unsafe extern "fastcall" fn(*mut FrameLimiter);
}

pub fn initialize(ctx: &PatchContext) -> Result<(), retour::Error> {
    unsafe {
        FrameLimiter_Update.initialize(transmute(ctx.offsets[CT3::FrameLimiterUpdate]), {
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
