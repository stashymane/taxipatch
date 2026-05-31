use crate::data::{PatchContext, CT3};
use crate::patch::Patch;
use game::{FrameLimiter, FrameLimiter_UpdateHook};
use std::mem::transmute;

inventory::submit! {
    Patch {
        name: "fps",
        priority: 0,
        register: initialize
    }
}

pub fn initialize(ctx: &PatchContext) -> anyhow::Result<()> {
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
