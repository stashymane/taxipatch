use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchSettings {
    #[serde(default = "enabled")]
    pub resolution: bool,
}

impl Default for PatchSettings {
    fn default() -> Self {
        Self {
            resolution: enabled(),
        }
    }
}

fn enabled() -> bool {
    true
}
