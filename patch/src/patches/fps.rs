use crate::data::PatchContext;
use crate::patch::Patch;
use game::FrameLimiter;

inventory::submit! {
    Patch::new("fps", initialize)
}

pub fn initialize(_ctx: &PatchContext) -> anyhow::Result<()> {
    unsafe {
        FrameLimiter::update.hook({
            move |fun, frame_limiter_ptr| {
                let frame_limiter: &mut FrameLimiter = &mut *frame_limiter_ptr;

                frame_limiter.adaptive_mode = false;

                fun.call((frame_limiter_ptr,));
            }
        });
    }
    Ok(())
}
