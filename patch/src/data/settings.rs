pub mod game;
pub mod patches;
pub mod general;

use crate::data::game::GameSettings;
use crate::data::general::GeneralSettings;
use crate::data::patches::PatchSettings;
use anyhow::Context;
use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::fs::{exists, write};

const CONFIG_FILE: &str = "taxipatch.ini";
const CONFIG_CONTENTS: &str = include_str!("../../resources/config.ini");

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    #[serde(default)]
    pub game: GameSettings,
    #[serde(default)]
    pub patches: PatchSettings,
    #[serde(default)]
    pub general: GeneralSettings,
}

impl Settings {
    pub fn load() -> anyhow::Result<Self> {
        if !exists(CONFIG_FILE)? {
            write(CONFIG_FILE, CONFIG_CONTENTS).context("Failed to save default config")?;
        }

        let config = Config::builder()
            .add_source(File::with_name(CONFIG_FILE).required(true))
            .build()?;

        let settings = config.try_deserialize().context("Config parsing failed")?;

        Ok(settings)
    }
}
