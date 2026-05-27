use crate::windows::display::get_display_info;
use anyhow::Context;
use config::{Config, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub window: WindowSettings,
    #[serde(default = "default_fps_limit")]
    pub fps_limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowSettings {
    #[serde(default = "default_resolution")]
    pub resolution: String,
    #[serde(default = "default_refresh_rate")]
    pub refresh_rate: u32,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            resolution: default_resolution(),
            refresh_rate: default_refresh_rate(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window: WindowSettings::default(),
            fps_limit: default_fps_limit(),
        }
    }
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

impl WindowSettings {
    pub fn resolution_u32(&self) -> Option<(u32, u32)> {
        let normalized = self.resolution.trim().to_ascii_lowercase();
        let mut parts = normalized.split('x');

        let width = parts.next()?.trim().parse::<u32>().ok()?;
        let height = parts.next()?.trim().parse::<u32>().ok()?;

        if parts.next().is_some() {
            return None;
        }

        if width == 0 || height == 0 {
            return None;
        }

        Some((width, height))
    }
}

fn default_resolution() -> String {
    let info = get_display_info();
    format!("{}x{}", info.width, info.height)
}

fn default_refresh_rate() -> u32 {
    get_display_info().refresh_rate
}

fn default_fps_limit() -> u32 {
    60
}
