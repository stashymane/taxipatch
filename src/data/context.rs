use crate::data::{Offsets, Settings};

#[derive(Debug)]
pub struct PatchContext {
    pub offsets: Offsets,
    pub settings: Settings,
}

impl PatchContext {
    pub fn from(offsets: Offsets, settings: Settings) -> anyhow::Result<PatchContext> {
        Ok(PatchContext { offsets, settings })
    }
}
