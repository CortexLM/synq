use subxt::{
    Config,
    SubstrateConfig,
};
use crate::subtensor::Subtensor;

pub mod axon;
pub mod rpc;
pub mod wallet;
pub mod weights;

pub mod subtensor;
pub mod sign;

include!(concat!(env!("OUT_DIR"), "/metadata.rs"));

pub type SubtensorConfig = SubstrateConfig;

pub type AccountId = <SubtensorConfig as Config>::AccountId;
pub type Hash = <SubtensorConfig as Config>::Hash;
pub type Header = <SubstrateConfig as Config>::Header;
pub type Block = subxt::blocks::Block<SubtensorConfig, Subtensor>;
pub type BlockRef = subxt::blocks::BlockRef<Hash>;
pub type BlockNumber = <Header as subxt::config::Header>::Number;
