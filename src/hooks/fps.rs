use crate::data::{PatchContext, CT3};
use crate::game::{FrameLimiter, FrameLimiter_UpdateHook};
use std::mem::transmute;

pub fn initialize(ctx: &PatchContext) -> Result<(), retour::Error> {
    unsafe {
        FrameLimiter_UpdateHook.initialize(transmute(ctx.offsets[CT3::FrameLimiterUpdate]), {
            move |frame_limiter_ptr| {
                let frame_limiter: &mut FrameLimiter = &mut *frame_limiter_ptr;

                frame_limiter.adaptive_mode = false;

                FrameLimiter_UpdateHook.call(frame_limiter_ptr);
            }
        })?;

        FrameLimiter_UpdateHook.enable()?;
    }
    Ok(())
}
