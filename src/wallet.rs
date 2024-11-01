use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use hex::FromHexError;
use serde_json::Value;
use subxt::ext::sp_core::{Pair, sr25519};
use subxt::ext::sp_core::crypto::SecretStringError;
use subxt::tx::PairSigner;
use thiserror::Error;
use crate::{AccountId, SubtensorConfig};

pub type Keypair = sr25519::Pair;
pub type PublicKey = sr25519::Public;
pub type Signer = PairSigner<SubtensorConfig, Keypair>;

#[derive(Error, Debug)]
pub struct InvalidAccountJsonError(PathBuf);

impl Display for InvalidAccountJsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Invalid wallet account file: {:?}", self.0))
    }
}

#[derive(Error, Debug)]
pub enum AccountLoadingError {
    #[error(transparent)]
    InvalidJson(#[from] InvalidAccountJsonError),

    #[error(transparent)]
    HexDecodeError(#[from] FromHexError),

    #[error(transparent)]
    JsonReaderError(#[from] serde_json::Error),

    #[error(transparent)]
    IoError(#[from] io::Error),
}

pub fn hotkey_location(
    mut wallet_path: PathBuf,
    wallet_name: impl AsRef<Path>,
    hotkey_name: impl AsRef<Path>,
) -> PathBuf {
    wallet_path.push(wallet_name);
    wallet_path.push("hotkeys");
    wallet_path.push(hotkey_name);

    wallet_path
}

pub fn home_hotkey_location(
    wallet_name: impl AsRef<Path>,
    hotkey_name: impl AsRef<Path>,
) -> Option<PathBuf> {
    dirs::home_dir().map(|mut wallet_path| {
        wallet_path.push(".bittensor");
        wallet_path.push("wallets");

        hotkey_location(wallet_path, wallet_name, hotkey_name)
    })
}

pub fn load_key_seed(path: impl AsRef<Path>) -> Result<[u8; 32], AccountLoadingError> {
    let json: Value = serde_json::from_reader(File::open(&path)?)?;

    let seed = json
        .as_object()
        .ok_or_else(|| InvalidAccountJsonError(path.as_ref().to_path_buf()))?
        .get("secretSeed")
        .ok_or_else(|| InvalidAccountJsonError(path.as_ref().to_path_buf()))?
        .as_str()
        .ok_or_else(|| InvalidAccountJsonError(path.as_ref().to_path_buf()))?;

    let mut decoded = [0; 32];
    hex::decode_to_slice(&seed[2..], &mut decoded)?;

    Ok(decoded)
}

pub fn load_key_account_id(path: impl AsRef<Path>) -> Result<AccountId, AccountLoadingError> {
    let json: Value = serde_json::from_reader(File::open(&path)?)?;

    let seed = json
        .as_object()
        .ok_or_else(|| InvalidAccountJsonError(path.as_ref().to_path_buf()))?
        .get("accountId")
        .ok_or_else(|| InvalidAccountJsonError(path.as_ref().to_path_buf()))?
        .as_str()
        .ok_or_else(|| InvalidAccountJsonError(path.as_ref().to_path_buf()))?;

    let mut decoded = [0; 32];
    hex::decode_to_slice(&seed[2..], &mut decoded)?;

    Ok(AccountId::from(decoded))
}

pub fn signer_from_seed(seed: &[u8]) -> Result<Signer, SecretStringError> {
    Ok(Signer::new(sr25519::Pair::from_seed_slice(seed)?))
}
