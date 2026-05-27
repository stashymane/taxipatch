use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameSettings {
    #[serde(default = "default_fps_limit")]
    pub fps_limit: u32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            fps_limit: default_fps_limit(),
        }
    }
}

fn default_fps_limit() -> u32 {
    60
}
