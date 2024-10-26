pub mod protocol;
pub mod neuron;
pub mod substrate;
pub mod utils;

// Re-export commonly used items
pub use protocol::{SubnetProtocol, SubnetRequest, SubnetResponse};
pub use neuron::{Neuron, BaseNeuron};
pub use utils::config::SubnetConfig;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");