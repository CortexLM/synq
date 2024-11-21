# Rusttensor

A low level Rust library for creating and interacting with Bittensor subnets, originally developed for [rule30 / Subnet 36](https://github.com/womboai/rule-30-solver). Built using [subxt](https://github.com/paritytech/subxt)

## Features

- Type safe runtime APIs such as neuron info, hyperparameters, stake info and delegate info
- Type safe extrinsics such as weight setting, axon serving, and much, _much_ more
- Easy access to storage without direct APIs
- Coldkey and hotkey loading utilities
- Auto-generated chain metadata types and functions to ensure everything is tested at compile-time
- Access to chain constants locally at compile-time with no network roundabout
- Full control and flexibility over transaction creation, signing and submission. Along with full flexibility over block hash handling to allow avoiding extra subtensor calls. 

## Prerequisites

- Rust toolchain (latest stable version)
- Cargo package manager
- Access to a Bittensor chain endpoint (such as `wss://entrypoint-finney.opentensor.ai:443`)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rusttensor = { git = "https://github.com/womboai/rusttensor", tag = "v0.3.0" }
```

## Usage Examples

### Creating a client and connecting to the subtensor

```rust
use rusttensor::subtensor::{self, Subtensor, SubtensorUrl};

async fn create_client() -> Result<Subtensor, ...> {
    // Creating client to interact with subtensor
    subtensor::from_url(SubtensorUrl::Finney).await;
}
```

### Unauthorized queries

#### Block Management

`rusttensor`, based on `subxt` allows all the functionality that `subxt` provides, including the blocks API. You can fetch metadata about any block, and reuse block hashes as needed.

```rust
use rusttensor::subtensor::Subtensor;

async fn blocks(client: &Subtensor) -> Result<(), ...> {
    // The latest block can be acquired with `blocks().at_latest()`
    let latest = client.blocks().at_latest().await?;

    // The block number, reference and hash can then be acquired
    let block_number = latest.header.number();
    let block_hash = latest.header.hash();
    let block_reference = latest.reference();

    // The reference can be used in selecting runtime APIs or storages to query
}
```

#### Runtime APIs
The most common requests to the subtensor aside from the weight setting and axon serving extrinsics are the runtime APIs used for the metagraph and hyperparameters

By default, most RPC calls at runtime are untyped(returning a Vec<u8>), 
as such, `rusttensor` provides a safety layer in the form of `call_runtime_api_decoded` which allows the runtime APIs to be completely type safe, much like the rest of the API.

```rust
use rusttensor::rpc::{call_runtime_api_decoded, NeuronInfoLite, SubnetHyperparams};
use rusttensor::api;

async fn runtime_apis(client: &Subtensor, block_ref: impl Into<BlockRef<impl BlockHash>>) -> Result<(), ...> {
    // Construct runtime API query for subnet 1 neuron info
    let neurons_lites_payload = api::apis().neuron_info_runtime_api().get_neurons_lite(1);

    // query NeuronInfoRuntimeApi at specific block
    let block_runtime = client.runtime_api().at(block_ref)?;
    let neurons: Vec<NeuronInfoLite> = call_runtime_api_decoded(&block_runtime, neurons_lites_payload).await?;

    // Or query hyperparameters for subnet 2 at latest block
    let hyper_parameters_payload = api::apis().subnet_info_runtime_api().get_subnet_info(2);
    let latest_runtime = client.runtime_api().at_latest().await?;
    let hyperparameters: Option<SubnetHyperparams> = call_runtime_api_decoded(&latest_runtime, hyper_parameters_payload).await?;

    // Some runtime APIs don't need decoding, such as the subnet registration cost API
    let payload = api::apis().subnet_registration_runtime_api().get_network_registration_cost();
    let registration_cost_in_rao = latest_runtime.call(payload).await?;
}
```

#### Storage
Some functionality doesn't have a specific API, such as neuron commitments which are used for arbitrary metadata like in SN39. In such cases, you can access the subtensor storage. 

```rust
use rusttensor::api;

async fn storage(client: &Subtensor) -> Result<(), ...> {
    let account_id: AccountId = ...;

    // Get the commitment :
    let commitment_address = api::storage().commitment_of(39, account_id);
    let latest_storage = client.storage().at_latest().await?;
    let commitment = latest_storage.fetch(commitment_address).await?;

    // Type safe access to commitment
}
```

### Authorized extrinsics

#### Wallet Management (WIP)
You can load existing bittensor wallets created using `btcli` and use them for signing extrinsics such as set_weights or serve_axon. Different kind of wallets can be loaded as follows:
```rust
use rusttensor::wallet::{Signer, home_hotkey_location, load_key_seed, signer_from_seed};

// Create a signer from the private key of a hotkey
fn load_hotkey_signer() -> Result<Signer, ...> {
    let path = home_hotkey_location("coldkey", "hotkey").expect("No home directory");
    let seed = load_key_seed(&path)?;// load seed for creating a signer

    Ok(signer_from_seed(&seed)?)
}

// Or just the account ID from the public key
fn load_hotkey_account_id() -> Result<Signer, ...> {
    let path = home_hotkey_location("coldkey", "hotkey").expect("No home directory");

    Ok(load_key_account_id(&path)?)
}
```

With a signer created in a similar fashion to `load_hotkey_signer`, we can submit extrinsics to the chain.

#### Submitting extrinsics
Some extrinsics have specialized APIs that are nicer to work with, such as `serve_axon` and `set_weights`, which reduce the number of parameters needed and uses more idiomatic types.
Regardless of if the extrinsic has a specialized API or otherwise, submitting them remains the same:

```rust
use rusttensor::subtensor::Subtensor;
use rusttensor::wallet::Signer;
use rusttensor::weights::{set_weights_payload, normalize_weights, NormalizedWeight};

async fn submit_extrinsics(client: &Subtensor, signer: &Signer) -> Result<(), ...> {
    let weights = vec![1.0, 2.0, 3.0];

    let weights = normalize_weights(&weights)
        .enumerate()
        .map(|(index, weight)| NormalizedWeight {
            uid: index as u16,
            weight,
        });

    let payload = set_weights_payload(
        1, // netuid
        weights,
        0, // version_key
    );

    // if your extrinsic doesn't have a wrapping API, it can be created using api::tx(), such as api::tx().subtensor_module().dissolve_network(30) for dissolving SN30

    // Submit the transaction
    let transaction = subtensor.client
        .tx()
        .sign_and_submit_then_watch_default(&payload, keypair) // or sign_and_submit_default to avoid waiting for inclusion
        .await?;

    // watch transaction if needed to wait for finalization

    Ok(())
}
```

## Building

```bash
cargo build --release
```

## Development

The project uses a build script (`build.rs`) to automatically generate Substrate metadata types at compile time. This ensures type safety and up-to-date chain compatibility.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Note on Security

Always handle wallet keys and sensitive information with care. Never share or commit private keys or seed phrases.
