use crate::data::{PatchContext, CT3};
use game::unassigned::GameTickHook;
use game::{
    BootLogoSequence_UpdateHook, BuildPresentParamsHook, CD3DApplication_InitWindowHook,
    FrameLimiter_UpdateHook, SetCameraPerspectiveHook,
};

pub fn init_hooks(ctx: &PatchContext) -> anyhow::Result<()> {
    unsafe {
        CD3DApplication_InitWindowHook
            .initialize_at(ctx.offsets[CT3::CD3DInitWindow])?
            .enable()?;
        BootLogoSequence_UpdateHook
            .initialize_at(ctx.offsets[CT3::BootLogoSequenceUpdate])?
            .enable()?;
        BuildPresentParamsHook
            .initialize_at(ctx.offsets[CT3::BuildPresentParams])?
            .enable()?;
        SetCameraPerspectiveHook
            .initialize_at(ctx.offsets[CT3::SetCameraPerspective])?
            .enable()?;

        FrameLimiter_UpdateHook
            .initialize_at(ctx.offsets[CT3::FrameLimiterUpdate])?
            .enable()?;

        GameTickHook
            .initialize_at(ctx.offsets[CT3::GameTick])?
            .enable()?;
    }
    Ok(())
}
