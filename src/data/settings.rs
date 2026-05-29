pub mod game;
pub mod patches;

use crate::data::game::GameSettings;
use anyhow::Context;
use config::{Config, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    #[serde(default)]
    pub game: GameSettings,
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
