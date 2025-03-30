# Synq

**Synq** is a high-level, developer-friendly Rust library for interacting with [Bittensor](https://bittensor.com) subnets. It is designed for flexibility, readability, and ease of integration. Synq is a refined fork of the original [`rusttensor`](https://github.com/womboai/rusttensor) project developed by **WUMBO AI**, adapted with improved modularity, structure, and documentation.

---

## ✨ Features

- ✅ Modular and well-documented Rust code
- ✅ Type-safe runtime APIs (e.g. neuron info, hyperparameters)
- ✅ Type-safe extrinsics (e.g. set weights, serve axon)
- ✅ Access to storage, runtime, constants, and extrinsics using `subxt`
- ✅ Wallet utilities for loading hotkeys/coldkeys
- ✅ Full control over block reference management and transaction submission

---

## 🚀 Getting Started

### 📦 Prerequisites

- Rust toolchain (latest stable)
- Cargo package manager
- Access to a Bittensor endpoint (e.g. `wss://entrypoint-finney.opentensor.ai:443`)

### 🔧 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
synq = { git = "https://github.com/your-username/synq" }
```

---

## 🧪 Usage Examples

### ✅ Connect to Subtensor

```rust
use synq::{client::SynqClient, network::NetworkEndpoint};

#[tokio::main]
async fn main() {
    let client = SynqClient::connect(NetworkEndpoint::Finney).await.unwrap();
    println!("Connected!");
}
```

### 🔑 Load Wallet & Sign

```rust
use synq::wallet::{load_seed_from_json, create_signer_from_seed};
use synq::signer::sign_message;

let seed = load_seed_from_json("./examples/my_wallet.json").unwrap();
let signer = create_signer_from_seed(&seed).unwrap();
let signature = sign_message(&signer, b"hello");
```

---

## 📚 Advanced: Runtime APIs & Storage

### Runtime Queries

```rust
use synq::rpc::{call_runtime_api_decoded, NeuronInfoLite};

let payload = api::apis().neuron_info_runtime_api().get_neurons_lite(1);
let block_runtime = client.runtime_api().at_latest().await?;
let neurons: Vec<NeuronInfoLite> = call_runtime_api_decoded(&block_runtime, payload).await?;
```

### Storage Access

```rust
let account_id: AccountId = ...;
let commitment_address = api::storage().commitment_of(39, account_id);
let storage = client.storage().at_latest().await?;
let commitment = storage.fetch(commitment_address).await?;
```

---

## 📤 Submitting Extrinsics

```rust
use synq::weights::{normalize_weights, set_weights_payload, NormalizedWeight};

let weights = normalize_weights(&vec![1.0, 2.0, 3.0])
    .enumerate()
    .map(|(uid, weight)| NormalizedWeight { uid: uid as u16, weight });

let payload = set_weights_payload(1, weights, 0);

let tx = client.inner()
    .tx()
    .sign_and_submit_then_watch_default(&payload, &signer)
    .await?;
```

---

## 🛠 Building

```bash
cargo build --release
```

---

## 📁 Project Structure

```text
src/
├── client.rs       # Substrate client connection
├── wallet.rs       # Wallet management
├── signer.rs       # Signing & verification
├── network.rs      # Subtensor endpoints
├── types.rs        # Shared type aliases
├── errors.rs       # Error handling
├── rpc/            # Runtime API + custom calls
└── main.rs         # Example CLI usage
```

---

## 📜 License

MIT License — forked and adapted from [WUMBO AI / rusttensor](https://github.com/womboai/rusttensor)

---

## 🤝 Contributing

We welcome contributions! Please open a pull request or start a discussion.

---

## ⚠️ Security Notice

Always store your coldkeys and hotkeys securely. Never share private seeds or commit sensitive information.