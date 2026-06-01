use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GameSettings {
    pub resolution: Option<String>,
    pub refresh_rate: Option<u32>,
    pub mode: Option<WindowMode>,
    pub fov: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Default, Copy, Clone, Eq, PartialEq)]
pub enum WindowMode {
    Fullscreen,
    #[default]
    Borderless,
    Windowed,
}
