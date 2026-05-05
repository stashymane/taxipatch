use crate::windows::display::get_display_info;
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
    pub fn load() -> Self {
        let config = Config::builder()
            .add_source(File::with_name("taxipatch.ini").required(false))
            .build()
            .unwrap();

        config.try_deserialize().unwrap()
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
