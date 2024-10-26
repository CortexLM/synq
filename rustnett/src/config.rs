use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetConfig {
    pub epoch_length: u64,
    pub chain_endpoint: String,
    pub netuid: u16,
    pub wallet_name: String,
    pub hotkey_name: String,
}

impl SubnetConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            epoch_length: std::env::var("EPOCH_LENGTH")
                .map(|var| var.parse().unwrap())
                .unwrap_or(100),
            chain_endpoint: std::env::var("CHAIN_ENDPOINT")?,
            netuid: std::env::var("NETUID")?.parse()?,
            wallet_name: std::env::var("WALLET_NAME")?,
            hotkey_name: std::env::var("HOTKEY_NAME")?,
        })
    }

    pub fn from_file(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }
}