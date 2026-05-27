pub mod game;
pub mod patches;
pub mod window;

use crate::data::game::GameSettings;
use crate::data::patches::PatchSettings;
use crate::data::window::WindowSettings;
use anyhow::Context;
use config::{Config, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub window: WindowSettings,
    #[serde(default)]
    pub game: GameSettings,
    #[serde(default)]
    pub patches: PatchSettings,
}

impl Settings {
    pub fn load() -> anyhow::Result<Self> {
        let config = Config::builder()
            .add_source(File::with_name("taxipatch.ini").required(false))
            .build()
            .context("Failed to load settings")?;

        config.try_deserialize().context("Failed to parse settings")
    }
}
