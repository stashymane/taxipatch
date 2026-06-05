use anyhow::Context;
use serde::{Deserialize, Serialize};
use windows::Win32::Graphics::Direct3D9::{
    D3DMULTISAMPLE_2_SAMPLES, D3DMULTISAMPLE_4_SAMPLES, D3DMULTISAMPLE_8_SAMPLES,
    D3DMULTISAMPLE_NONE, D3DMULTISAMPLE_TYPE,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GameSettings {
    pub resolution: Option<String>,
    pub refresh_rate: Option<u32>,
    pub mode: Option<WindowMode>,
    pub buffering_mode: Option<BufferingMode>,
    pub multisampling: Option<MultisamplingMode>,

    pub fov: Option<f32>,
}

impl GameSettings {
    pub fn resolution_tuple<F>(&self, default: F) -> anyhow::Result<(u32, u32)>
    where
        F: FnOnce() -> (u32, u32),
    {
        match &self.resolution {
            Some(resolution) => resolution
                .split('x')
                .map(|dim| dim.parse::<u32>().context("Failed to parse resolution"))
                .collect::<anyhow::Result<Vec<_>>>()
                .map(|result| (result[0], result[1])),
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

#[derive(Debug, Serialize, Deserialize, Default, Copy, Clone)]
pub enum BufferingMode {
    // Double has a bug where scene changes retain the last frame, needs to be fixed before it's default
    Double,
    #[default]
    Triple,
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
