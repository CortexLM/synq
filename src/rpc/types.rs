use crate::AccountId;
use parity_scale_codec::{Compact, Decode, Encode};

#[derive(Decode, Encode, PartialEq, Eq, Clone, Debug)]
pub struct DelegateInfo {
    pub delegate_ss58: AccountId,
    pub take: Compact<u16>,
    pub nominators: Vec<(AccountId, Compact<u64>)>, // map of nominator_ss58 to stake amount
    pub owner_ss58: AccountId,
    pub registrations: Vec<Compact<u16>>, // Vec of netuid this delegate is registered on
    pub validator_permits: Vec<Compact<u16>>, // Vec of netuid this delegate has validator permit on
    pub return_per_1000: Compact<u64>, // Delegators current daily return per 1000 TAO staked minus take fee
    pub total_daily_return: Compact<u64>, // Delegators current daily return
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct AxonInfo {
    ///  Axon serving block.
    pub block: u64,
    ///  Axon version
    pub version: u32,
    ///  Axon u128 encoded ip address of type v6 or v4.
    pub ip: u128,
    ///  Axon u16 encoded port.
    pub port: u16,
    ///  Axon ip type, 4 for ipv4 and 6 for ipv6.
    pub ip_type: u8,
    ///  Axon protocol. TCP, UDP, other.
    pub protocol: u8,
    ///  Axon proto placeholder 1.
    pub placeholder1: u8,
    ///  Axon proto placeholder 2.
    pub placeholder2: u8,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct PrometheusInfo {
    /// Prometheus serving block.
    pub block: u64,
    /// Prometheus version.
    pub version: u32,
    ///  Prometheus u128 encoded ip address of type v6 or v4.
    pub ip: u128,
    ///  Prometheus u16 encoded port.
    pub port: u16,
    /// Prometheus ip type, 4 for ipv4 and 6 for ipv6.
    pub ip_type: u8,
}

#[derive(Decode, Encode, PartialEq, Eq, Clone, Debug)]
pub struct NeuronInfo {
    pub hotkey: AccountId,
    pub coldkey: AccountId,
    pub uid: Compact<u16>,
    pub netuid: Compact<u16>,
    pub active: bool,
    pub axon_info: AxonInfo,
    pub prometheus_info: PrometheusInfo,
    pub stake: Vec<(AccountId, Compact<u64>)>, // map of coldkey to stake on this neuron/hotkey (includes delegations)
    pub rank: Compact<u16>,
    pub emission: Compact<u64>,
    pub incentive: Compact<u16>,
    pub consensus: Compact<u16>,
    pub trust: Compact<u16>,
    pub validator_trust: Compact<u16>,
    pub dividends: Compact<u16>,
    pub last_update: Compact<u64>,
    pub validator_permit: bool,
    pub weights: Vec<(Compact<u16>, Compact<u16>)>, // Vec of (uid, weight)
    pub bonds: Vec<(Compact<u16>, Compact<u16>)>,   // Vec of (uid, bond)
    pub pruning_score: Compact<u16>,
}

#[derive(Decode, Encode, PartialEq, Eq, Clone, Debug)]
pub struct NeuronInfoLite {
    pub hotkey: AccountId,
    pub coldkey: AccountId,
    pub uid: Compact<u16>,
    pub netuid: Compact<u16>,
    pub active: bool,
    pub axon_info: AxonInfo,
    pub prometheus_info: PrometheusInfo,
    pub stake: Vec<(AccountId, Compact<u64>)>, // map of coldkey to stake on this neuron/hotkey (includes delegations)
    pub rank: Compact<u16>,
    pub emission: Compact<u64>,
    pub incentive: Compact<u16>,
    pub consensus: Compact<u16>,
    pub trust: Compact<u16>,
    pub validator_trust: Compact<u16>,
    pub dividends: Compact<u16>,
    pub last_update: Compact<u64>,
    pub validator_permit: bool,
    // has no weights or bonds
    pub pruning_score: Compact<u16>,
}

#[derive(Decode, Encode, PartialEq, Eq, Clone, Debug)]
pub struct StakeInfo {
    pub hotkey: AccountId,
    pub coldkey: AccountId,
    pub stake: Compact<u64>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct SubnetIdentity {
    /// The name of the subnet
    pub subnet_name: Vec<u8>,
    /// The github repository associated with the chain identity
    pub github_repo: Vec<u8>,
    /// The subnet's contact
    pub subnet_contact: Vec<u8>,
}

#[derive(Decode, Encode, PartialEq, Eq, Clone, Debug)]
pub struct SubnetInfo {
    pub netuid: Compact<u16>,
    pub rho: Compact<u16>,
    pub kappa: Compact<u16>,
    pub difficulty: Compact<u64>,
    pub immunity_period: Compact<u16>,
    pub max_allowed_validators: Compact<u16>,
    pub min_allowed_weights: Compact<u16>,
    pub max_weights_limit: Compact<u16>,
    pub scaling_law_power: Compact<u16>,
    pub subnetwork_n: Compact<u16>,
    pub max_allowed_uids: Compact<u16>,
    pub blocks_since_last_step: Compact<u64>,
    pub tempo: Compact<u16>,
    pub network_modality: Compact<u16>,
    pub network_connect: Vec<[u16; 2]>,
    pub emission_values: Compact<u64>,
    pub burn: Compact<u64>,
    pub owner: AccountId,
}

#[derive(Decode, Encode, PartialEq, Eq, Clone, Debug)]
pub struct SubnetInfov2 {
    pub netuid: Compact<u16>,
    pub rho: Compact<u16>,
    pub kappa: Compact<u16>,
    pub difficulty: Compact<u64>,
    pub immunity_period: Compact<u16>,
    pub max_allowed_validators: Compact<u16>,
    pub min_allowed_weights: Compact<u16>,
    pub max_weights_limit: Compact<u16>,
    pub scaling_law_power: Compact<u16>,
    pub subnetwork_n: Compact<u16>,
    pub max_allowed_uids: Compact<u16>,
    pub blocks_since_last_step: Compact<u64>,
    pub tempo: Compact<u16>,
    pub network_modality: Compact<u16>,
    pub network_connect: Vec<[u16; 2]>,
    pub emission_values: Compact<u64>,
    pub burn: Compact<u64>,
    pub owner: AccountId,
    pub identity: Option<SubnetIdentity>,
}

#[derive(Decode, Encode, PartialEq, Eq, Clone, Debug)]
pub struct SubnetHyperparams {
    pub rho: Compact<u16>,
    pub kappa: Compact<u16>,
    pub immunity_period: Compact<u16>,
    pub min_allowed_weights: Compact<u16>,
    pub max_weights_limit: Compact<u16>,
    pub tempo: Compact<u16>,
    pub min_difficulty: Compact<u64>,
    pub max_difficulty: Compact<u64>,
    pub weights_version: Compact<u64>,
    pub weights_rate_limit: Compact<u64>,
    pub adjustment_interval: Compact<u16>,
    pub activity_cutoff: Compact<u16>,
    pub registration_allowed: bool,
    pub target_regs_per_interval: Compact<u16>,
    pub min_burn: Compact<u64>,
    pub max_burn: Compact<u64>,
    pub bonds_moving_avg: Compact<u64>,
    pub max_regs_per_block: Compact<u16>,
    pub serving_rate_limit: Compact<u64>,
    pub max_validators: Compact<u16>,
    pub adjustment_alpha: Compact<u64>,
    pub difficulty: Compact<u64>,
    pub commit_reveal_weights_interval: Compact<u64>,
    pub commit_reveal_weights_enabled: bool,
    pub alpha_high: Compact<u16>,
    pub alpha_low: Compact<u16>,
    pub liquid_alpha_enabled: bool,
}
