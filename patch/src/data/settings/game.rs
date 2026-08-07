use serde::{Deserialize, Serialize};
use thiserror::Error;
use windows::Win32::Graphics::Direct3D9::{
    D3DMULTISAMPLE_2_SAMPLES, D3DMULTISAMPLE_4_SAMPLES, D3DMULTISAMPLE_8_SAMPLES,
    D3DMULTISAMPLE_NONE, D3DMULTISAMPLE_TYPE,
};

#[derive(Debug, Error)]
pub enum ResolutionError {
    #[error("resolution must be [WIDTH]x[HEIGHT]")]
    InvalidFormat,
    #[error("Failed to parse resolution width")]
    InvalidWidth(#[source] std::num::ParseIntError),
    #[error("Failed to parse resolution height")]
    InvalidHeight(#[source] std::num::ParseIntError),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GameSettings {
    pub resolution: Option<String>,
    pub refresh_rate: Option<u32>,
    pub mode: Option<WindowMode>,
    pub multisampling: Option<MultisamplingMode>,
    pub fov: Option<f32>,
}

impl GameSettings {
    pub fn resolution_tuple<F>(&self, default: F) -> Result<(u32, u32), ResolutionError>
    where
        F: FnOnce() -> (u32, u32),
    {
        match &self.resolution {
            Some(resolution) => {
                let mut parts = resolution.split('x');
                let width = parts
                    .next()
                    .ok_or(ResolutionError::InvalidFormat)?
                    .parse()
                    .map_err(ResolutionError::InvalidWidth)?;
                let height = parts
                    .next()
                    .ok_or(ResolutionError::InvalidFormat)?
                    .parse()
                    .map_err(ResolutionError::InvalidHeight)?;
                if parts.next().is_some() {
                    return Err(ResolutionError::InvalidFormat);
                }
                Ok((width, height))
            }
            None => Ok(default()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Copy, Clone, Eq, PartialEq)]
pub enum WindowMode {
    Fullscreen,
    #[default]
    Borderless,
    Windowed,
}

#[derive(Debug, Serialize, Deserialize, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MultisamplingMode {
    #[default]
    Off,
    X2,
    X4,
    X8,
}

impl Into<D3DMULTISAMPLE_TYPE> for MultisamplingMode {
    fn into(self) -> D3DMULTISAMPLE_TYPE {
        match self {
            MultisamplingMode::Off => D3DMULTISAMPLE_NONE,
            MultisamplingMode::X2 => D3DMULTISAMPLE_2_SAMPLES,
            MultisamplingMode::X4 => D3DMULTISAMPLE_4_SAMPLES,
            MultisamplingMode::X8 => D3DMULTISAMPLE_8_SAMPLES,
        }
    }
}
