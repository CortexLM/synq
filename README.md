# Bittensor Subnet Template

A Rust library template for creating and interacting with Bittensor subnets. This template provides the essential building blocks for developing subnet implementations with built-in chain communication capabilities.

## Features

- Built-in Substrate client implementation for Bittensor chain interaction
- Neuron management and information retrieval
- Weight setting capabilities
- Axon serving functionality
- Wallet key management utilities
- Auto-generated chain metadata types

## Prerequisites

- Rust toolchain (latest stable version)
- Cargo package manager
- Access to a Bittensor chain endpoint (default: wss://entrypoint-finney.opentensor.ai:443)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bittensor-subnet-template = "0.1.0"
```

## Usage

### Creating a Subtensor Client

```rust
use bittensor_subnet_template::Subtensor;

async fn example() -> anyhow::Result<()> {
    // Connect to the Bittensor network
    let subtensor = Subtensor::new("wss://entrypoint-finney.opentensor.ai:443").await?;
    
    // Get the current block number
    let block_number = subtensor.get_block_number().await?;
    println!("Current block: {}", block_number);
    
    Ok(())
}
```

### Retrieving Neuron Information

```rust
async fn get_subnet_neurons(subtensor: &Subtensor) -> anyhow::Result<()> {
    // Get all neurons for subnet 1
    let neurons = subtensor.get_neurons(1).await?;
    
    for neuron in neurons {
        println!("Neuron UID: {:?}", neuron.uid);
        println!("Active: {}", neuron.active);
        println!("Stake: {:?}", neuron.stake);
    }
    
    Ok(())
}
```

### Setting Weights

```rust
use bittensor_subnet_template::{Subtensor, WeightSet};

async fn set_neuron_weights(subtensor: &Subtensor, keypair: &Keypair) -> anyhow::Result<()> {
    let weights = vec![
        WeightSet { uid: 0, weight: 100 },
        WeightSet { uid: 1, weight: 200 },
    ];
    
    let payload = Subtensor::set_weights(
        1, // netuid
        weights,
        0, // version_key
    );
    
    // Submit the transaction
    subtensor.client
        .tx()
        .sign_and_submit_then_watch_default(&payload, keypair)
        .await?;
    
    Ok(())
}
```

### Managing Hotkeys

```rust
use bittensor_subnet_template::{hotkey_location, load_key_seed, Keypair};

fn load_hotkey() -> anyhow::Result<Keypair> {
    // Get the default hotkey path
    let hotkey_path = hotkey_location("default", "default")?;
    
    // Load the key seed
    let seed = load_key_seed(hotkey_path)?;
    
    // Create a keypair from the seed
    Keypair::from_seed(&seed)
}
```

## Configuration

The template can be configured by setting the following environment variable:

- `CHAIN_ENDPOINT`: The WebSocket endpoint for the Bittensor network (default: wss://entrypoint-finney.opentensor.ai:443)

## Building

```bash
cargo build --release
```

## Development

The project uses a build script (`build.rs`) to automatically generate Substrate metadata types at compile time. This ensures type safety and up-to-date chain compatibility.

### Custom Chain Endpoint

To use a different chain endpoint during development:

```bash
CHAIN_ENDPOINT=wss://your-endpoint.com cargo build
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Note on Security

Always handle wallet keys and sensitive information with care. Never share or commit private keys or seed phrases.
