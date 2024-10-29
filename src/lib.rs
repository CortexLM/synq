use std::fmt::{Display, Formatter};
use std::fs::File;
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

// Users can define their own config module (src/config.rs or src/config/mod.rs)
// to customize network configurations as needed. The config module can contain:
// - Network endpoints
// - Chain-specific parameters
// - Custom configuration types
// Example: mod config { pub const DEFAULT_ENDPOINT: &str = "ws://..."; }

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
    pub protocol: u8,
    _placeholder1: u8,
    _placeholder2: u8,
}

#[derive(Decode, Encode, Default, Clone, Debug)]
pub struct PrometheusInfo {
    pub block: u64,
    pub version: u32,
    pub ip: u128,
    pub port: u16,
    pub ip_type: u8,
}

#[derive(Decode, Encode, Clone, Debug)]
pub struct NeuronInfoLite {
    pub hotkey: AccountId,
    pub coldkey: AccountId,
    pub uid: Compact<u16>,
    pub netuid: Compact<u16>,
    pub active: bool,
    pub axon_info: AxonInfo,
    pub prometheus_info: PrometheusInfo,
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
pub struct SetWeightsCall {
    pub netuid: u16,
    pub uids: Vec<u16>,
    pub weights: Vec<u16>,
    pub version_key: u64,
}

#[derive(Debug)]
pub struct ServeAxonCall {
    pub netuid: u16,
    pub version: u32,
    pub ip: u128,
    pub port: u16,
    pub ip_type: u8,
    pub protocol: u8,
    pub placeholder1: u8,
    pub placeholder2: u8,
}

#[derive(Debug)]
pub struct ServePrometheusCall {
    pub netuid: u16,
    pub version: u32,
    pub ip: u128,
    pub port: u16,
    pub ip_type: u8,
}

pub struct Subtensor {
    client: OnlineClient<SubtensorConfig>,
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

    pub fn build_set_weights(&self, params: SetWeightsCall) -> impl Payload {
        api::tx().subtensor_module().set_weights(
            params.netuid,
            params.uids,
            params.weights,
            params.version_key,
        )
    }

    pub fn build_serve_axon(&self, params: ServeAxonCall) -> impl Payload {
        api::tx().subtensor_module().serve_axon(
            params.netuid,
            params.version,
            params.ip,
            params.port,
            params.ip_type,
            params.protocol,
            params.placeholder1,
            params.placeholder2,
        )
    }

    pub fn build_serve_prometheus(&self, params: ServePrometheusCall) -> impl Payload {
        api::tx().subtensor_module().serve_prometheus(
            params.netuid,
            params.version,
            params.ip,
            params.port,
            params.ip_type,
        )
    }

    pub async fn submit_transaction<T: Payload>(
        &self,
        payload: T,
        signer: &Keypair,
    ) -> Result<()> {
        self.client
            .tx()
            .sign_and_submit_default(&payload, &signer.0)
            .await?;

        Ok(())
    }

    pub async fn set_weights(
        &self,
        keypair: &Keypair,
        params: SetWeightsCall,
    ) -> Result<()> {
        let payload = self.build_set_weights(params);
        self.submit_transaction(payload, keypair).await
    }

    pub async fn serve_axon(
        &self,
        keypair: &Keypair,
        params: ServeAxonCall,
    ) -> Result<()> {
        let payload = self.build_serve_axon(params);
        self.submit_transaction(payload, keypair).await
    }

    pub async fn serve_prometheus(
        &self,
        keypair: &Keypair,
        params: ServePrometheusCall,
    ) -> Result<()> {
        let payload = self.build_serve_prometheus(params);
        self.submit_transaction(payload, keypair).await
    }

    pub async fn get_block_number(&self) -> Result<u64> {
        Ok(self.client.blocks().at_latest().await?.number().into())
    }
}