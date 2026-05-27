use crate::data::{GameType, Offsets, Settings};

#[derive(Debug)]
pub struct PatchContext {
    pub game_type: GameType,
    pub settings: Settings,
    pub offsets: Offsets,
}

impl PatchContext {
    pub fn load() -> anyhow::Result<PatchContext> {
        let game_type = GameType::load()?;
        let offsets = Offsets::from(&game_type)?;

        Ok(PatchContext {
            game_type,
            settings: Settings::load()?,
            offsets,
        })
    }
}
