use crate::windows::display::get_display_info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowSettings {
    #[serde(default = "default_resolution")]
    pub resolution: String,
    #[serde(default = "default_refresh_rate")]
    pub refresh_rate: u32,
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

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            resolution: default_resolution(),
            refresh_rate: default_refresh_rate(),
        }
    }
}

fn default_resolution() -> String {
    let info = get_display_info();
    format!("{}x{}", info.width, info.height)
}

fn default_refresh_rate() -> u32 {
    get_display_info().refresh_rate
}
