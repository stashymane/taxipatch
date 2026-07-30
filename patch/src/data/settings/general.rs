use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralSettings {
    #[serde(default = "enabled")]
    pub ignore_checksum_mismatch: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            ignore_checksum_mismatch: false
        }
    }
}

fn enabled() -> bool {
    true
}
