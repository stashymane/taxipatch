use crate::data::PatchContext;
use crate::patch::Patch;
use game::{FrameLimiter, FrameLimiter_UpdateHook};

inventory::submit! {
    Patch {
        name: "fps",
        priority: 0,
        register: initialize
    }
}

pub fn initialize(_ctx: &PatchContext) -> anyhow::Result<()> {
    unsafe {
        FrameLimiter_UpdateHook.wrap({
            move |fun, frame_limiter_ptr| {
                let frame_limiter: &mut FrameLimiter = &mut *frame_limiter_ptr;

                frame_limiter.adaptive_mode = false;

                fun.call(frame_limiter_ptr);
            }
        })?;
    }
    Ok(())
}
