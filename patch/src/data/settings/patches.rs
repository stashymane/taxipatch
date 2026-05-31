use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatchSettings {
    #[serde(default = "enabled")]
    pub skip_intro: bool,
}

impl Default for PatchSettings {
    fn default() -> Self {
        Self {
            skip_intro: enabled(),
        }
    }
}

fn enabled() -> bool {
    true
}
