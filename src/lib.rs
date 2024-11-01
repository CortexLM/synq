use node_subtensor_runtime::Runtime;
use subxt::{
    Config,
    SubstrateConfig,
};

pub mod axon;
pub mod rpc;
pub mod wallet;
pub mod weights;

pub mod subtensor;

include!(concat!(env!("OUT_DIR"), "/metadata.rs"));

pub type SubtensorConfig = SubstrateConfig;
pub type SpSubtensorConfig = Runtime;

pub type AccountId = <SubtensorConfig as Config>::AccountId;
