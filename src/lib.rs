use std::fmt::{Display, Formatter};
use std::fs::File;
use std::net::IpAddr;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use anyhow::Result;
use hex;
use parity_scale_codec::{Compact, Decode, Encode};
use serde_json::Value;
use subxt::client::OnlineClient;
use subxt::ext::sp_core::{sr25519, Pair};
use subxt::{
    SubstrateConfig,
    Config,
    tx::{PairSigner, Payload},
};
use thiserror::Error;

include!(concat!(env!("OUT_DIR"), "/metadata.rs"));

type SubtensorConfig = SubstrateConfig;
pub type AccountId = <SubtensorConfig as Config>::AccountId;

#[derive(Decode, Encode, Default, Clone, Debug)]
pub struct AxonInfo {
    pub block: u64,
    pub version: u32,
    pub ip: u128,
    pub port: u16,
    pub ip_type: u8,
    pub protocol: bool,
}

#[derive(Decode, Encode, Clone, Debug)]
pub struct NeuronInfoLite {
    pub hotkey: AccountId,
    pub coldkey: AccountId,
    pub uid: Compact<u16>,
    pub netuid: Compact<u16>,
    pub active: bool,
    pub axon_info: AxonInfo,
    pub stake: Vec<(AccountId, Compact<u64>)>,
    pub rank: Compact<u16>,
    pub emission: Compact<u64>,
    pub incentive: Compact<u16>,
    pub consensus: Compact<u16>,
    pub trust: Compact<u16>,
    pub validator_trust: Compact<u16>,
    pub dividends: Compact<u16>,
    pub last_update: Compact<u64>,
    pub validator_permit: bool,
    pub pruning_score: Compact<u16>,
}

#[derive(Debug)]
pub struct WeightSet {
    pub uid: u16,
    pub weight: u16,
}

pub struct Subtensor {
    pub client: OnlineClient<SubtensorConfig>,
}

#[derive(Error, Debug)]
struct InvalidAccountJsonError(PathBuf);

impl Display for InvalidAccountJsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Invalid wallet account file: {:?}", self.0))
    }
}

pub struct Keypair(PairSigner<SubtensorConfig, sr25519::Pair>);

impl Deref for Keypair {
    type Target = PairSigner<SubtensorConfig, sr25519::Pair>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Keypair {
    pub fn from_seed(seed: &[u8]) -> Result<Self> {
        Ok(Self(PairSigner::new(sr25519::Pair::from_seed_slice(seed)?)))
    }
}

impl Subtensor {
    pub async fn new(url: impl AsRef<str>) -> Result<Self> {
        Ok(Self {
            client: OnlineClient::from_url(url).await?,
        })
    }

    pub async fn get_neurons(&self, netuid: u16) -> Result<Vec<NeuronInfoLite>> {
        let neurons_payload = api::apis()
            .neuron_info_runtime_api()
            .get_neurons_lite(netuid);

        let response = self
            .client
            .runtime_api()
            .at_latest()
            .await?
            .call(neurons_payload)
            .await?;

        Ok(Vec::<NeuronInfoLite>::decode(&mut response.as_ref())?)
    }

    pub async fn get_block_number(&self) -> Result<u64> {
        Ok(self.client.blocks().at_latest().await?.number().into())
    }

    // Static payload builder functions
    pub fn set_weights(netuid: u16, weights: Vec<WeightSet>, version_key: u64) -> impl Payload {
        let (uids, weight_values): (Vec<_>, Vec<_>) = weights.into_iter()
            .map(|w| (w.uid, w.weight))
            .unzip();

        api::tx().subtensor_module().set_weights(
            netuid,
            uids,
            weight_values,
            version_key,
        )
    }

    pub fn serve_axon(netuid: u16, ip: IpAddr, port: u16, protocol: bool) -> impl Payload {
        let (ip_num, ip_type) = match ip {
            IpAddr::V4(addr) => (u128::from(u32::from(addr)), 4u8),
            IpAddr::V6(addr) => (u128::from(addr), 6u8),
        };

        api::tx().subtensor_module().serve_axon(
            netuid,
            1, // version is always 1 in practice
            ip_num,
            port,
            ip_type,
            protocol as u8,
            0, // placeholder1 unused
            0, // placeholder2 unused
        )
    }
}

// Wallet utility functions
pub fn hotkey_location_with_home(
    mut home_directory: PathBuf,
    wallet_name: impl AsRef<Path>,
    hotkey_name: impl AsRef<Path>,
) -> PathBuf {
    home_directory.push(".bittensor");
    home_directory.push("wallets");
    home_directory.push(wallet_name);
    home_directory.push("hotkeys");
    home_directory.push(hotkey_name);

    home_directory
}

pub fn hotkey_location(
    wallet_name: impl AsRef<Path>,
    hotkey_name: impl AsRef<Path>,
) -> Option<PathBuf> {
    Some(hotkey_location_with_home(
        dirs::home_dir()?,
        wallet_name,
        hotkey_name,
    ))
}

pub fn load_key_seed(path: impl AsRef<Path>) -> Result<[u8; 32]> {
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