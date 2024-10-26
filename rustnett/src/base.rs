use crate::substrate::chain::Subtensor;
use crate::utils::config::SubnetConfig;
use async_trait::async_trait;

#[async_trait]
pub trait Neuron: Send + Sync {
    type Error;

    /// Initialize the neuron
    async fn init(&mut self) -> Result<(), Self::Error>;

    /// Start the neuron's main loop
    async fn run(&mut self) -> Result<(), Self::Error>;

    /// Get the neuron's current score
    async fn get_score(&self) -> Result<u16, Self::Error>;

    /// Sync with the network
    async fn sync(&mut self) -> Result<(), Self::Error>;
}

pub struct BaseNeuron {
    pub config: SubnetConfig,
    pub subtensor: Subtensor,
    pub step: u64,
}