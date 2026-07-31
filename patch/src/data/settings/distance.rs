use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DistanceSettings {
    pub map: Option<f32>,
}
