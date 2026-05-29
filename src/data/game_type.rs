use crate::data::Offsets;
use anyhow::{anyhow, Context};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

const FAIRLIGHT_SHA256: &str = "9ad9ad77c7cab751529f03da807d3846887a47b89f48f0792cd9477f90b3a0d8";
const CT3CONFIG_SHA256: &str = "c5d89ace133713a9a5ec7068ded5681b715753ee21157a0f6107dccdbd6a89cd";

#[derive(Debug)]
pub enum ExecutableType {
    Fairlight(Offsets),
    Config,
}

impl ExecutableType {
    pub fn load() -> anyhow::Result<ExecutableType> {
        let hash = calculate_hash(std::env::current_exe()?)
            .context("Failed to calculate executable checksum")?;
        Self::from_hash(hash.as_str())
    }

    fn from_hash(hash: &str) -> anyhow::Result<ExecutableType> {
        match hash {
            FAIRLIGHT_SHA256 => Ok(ExecutableType::Fairlight(Offsets::get_fairlight()?)),
            CT3CONFIG_SHA256 => Ok(ExecutableType::Config),
            _ => Err(anyhow!(
                "Game executable is not supported.\r\nIf you have used CT3Tweaks on this executable before, restore the backup before running the game."
            )),
        }
    }
}

fn calculate_hash(path: PathBuf) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();

    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }

        Digest::update(&mut hasher, &buf[..n]);
    }

    let hash = hex::encode(hasher.finalize());

    Ok(hash)
}
