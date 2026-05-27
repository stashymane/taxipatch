use crate::data::GameType;
use anyhow::Context;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

#[derive(Debug)]
pub struct Offsets {
    pub base: usize,

    pub resolution_width: usize,
    pub resolution_height: usize,
    pub resolution_continuation: usize,

    pub intro_ticks: usize,
    pub init_game_state: usize,
    pub intro_state_value: usize,
}

impl Offsets {
    pub fn from(game_type: &GameType) -> anyhow::Result<Offsets> {
        match game_type {
            GameType::Fairlight => Self::get_fairlight(),
        }
    }

    pub fn get_fairlight() -> anyhow::Result<Offsets> {
        let base = unsafe {
            GetModuleHandleA(None)
                .context("Failed to retrieve module handle")?
                .0 as usize
        };

        Ok(Offsets {
            base,

            resolution_width: 0x001EC5F8,
            resolution_height: 0x001EC5FC,
            resolution_continuation: 0x00007A97,

            intro_ticks: 0x000734D0,
            init_game_state: 0x00073570,
            intro_state_value: 0x003BD2F0,
        })
    }
}
