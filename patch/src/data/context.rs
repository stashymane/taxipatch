use crate::data::Settings;

#[derive(Debug)]
pub struct PatchContext {
    pub settings: Settings,
}

impl PatchContext {
    pub fn from(settings: Settings) -> anyhow::Result<PatchContext> {
        Ok(PatchContext { settings })
    }
}
