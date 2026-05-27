use anyhow::{anyhow, Context};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

const FAIRLIGHT_SHA256: &str = "9ad9ad77c7cab751529f03da807d3846887a47b89f48f0792cd9477f90b3a0d8";

#[derive(Debug)]
pub enum GameType {
    Fairlight,
}

impl GameType {
    pub fn load() -> anyhow::Result<GameType> {
        let hash = calculate_hash(PathBuf::from("../CT3.exe"))
            .context("Failed to calculate game checksum")?;
        Self::from_hash(hash.as_str())
    }

    fn from_hash(hash: &str) -> anyhow::Result<GameType> {
        match hash {
            FAIRLIGHT_SHA256 => Ok(GameType::Fairlight),
            _ => Err(anyhow!("Game executable is not supported")),
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
