use crate::windows::display::get_display_info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameSettings {
    #[serde(default = "default_resolution")]
    pub resolution: String,
    #[serde(default = "default_refresh_rate")]
    pub refresh_rate: u32,
    #[serde(default = "default_window_mode")]
    pub mode: WindowMode,

    #[serde(default = "default_fps_limit")]
    pub fps_limit: u32,
    #[serde(default = "default_fov")]
    pub fov: f32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum WindowMode {
    Fullscreen,
    #[default]
    Borderless,
    Windowed,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            resolution: default_resolution(),
            refresh_rate: default_refresh_rate(),
            mode: default_window_mode(),
            fps_limit: default_fps_limit(),
            fov: default_fov(),
        }
    }
}

impl GameSettings {
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

    pub fn aspect_ratio(&self) -> Option<f32> {
        let (x, y) = self.resolution_u32()?;
        let aspect = x as f32 / y as f32;
        Some(aspect)
    }
}

fn default_resolution() -> String {
    let info = get_display_info();
    format!("{}x{}", info.width, info.height)
}

fn default_refresh_rate() -> u32 {
    get_display_info().refresh_rate
}
fn default_window_mode() -> WindowMode {
    WindowMode::Borderless
}
fn default_fps_limit() -> u32 {
    60
}
fn default_fov() -> f32 {
    90.0
}
