use crate::data::pointers::Pointers;
use crate::data::{Offsets, Settings};

#[derive(Debug)]
pub struct PatchContext {
    pub offsets: Offsets,
    pub pointers: Pointers,
    pub settings: Settings,
}

impl PatchContext {
    pub fn from(offsets: Offsets, settings: Settings) -> anyhow::Result<PatchContext> {
        let globals = Pointers::from(offsets.base);
        Ok(PatchContext {
            offsets,
            pointers: globals,
            settings,
        })
    }
}
