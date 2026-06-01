use crate::windows::display::get_display_info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GameSettings {
    pub resolution: Option<String>,
    pub refresh_rate: Option<u32>,
    pub mode: Option<WindowMode>,
    pub fov: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Default, Copy, Clone)]
pub enum WindowMode {
    Fullscreen,
    #[default]
    Borderless,
    Windowed,
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
fn default_fov() -> f32 {
    90.0
}
