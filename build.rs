use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;
use std::{env, process::Command};

use parity_scale_codec::Decode;
use subxt_codegen;
use subxt_codegen::CodegenBuilder;
use subxt_metadata::Metadata;
use subxt_utils_fetchmetadata::{self as fetch_metadata, MetadataVersion};

#[tokio::main]
async fn main() {
    let endpoint = env::var_os("METADATA_CHAIN_ENDPOINT")
        .map(|s| s.into_string().unwrap())
        .unwrap_or("wss://entrypoint-finney.opentensor.ai:443".into());

    let endpoint: &str = &endpoint;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let metadata_path = Path::new(&out_dir).join("metadata.rs");

    let metadata_bytes =
        fetch_metadata::from_url(endpoint.try_into().unwrap(), MetadataVersion::Latest)
            .await
            .unwrap();
    let mut metadata_bytes: &[u8] = &metadata_bytes;
    let metadata = Metadata::decode(&mut metadata_bytes).unwrap();

    let codegen = CodegenBuilder::new();

    let code = codegen.generate(metadata).unwrap();
    let file_output = File::create(metadata_path).unwrap();

    let mut process = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(file_output)
        .spawn()
        .unwrap();

    write!(process.stdin.as_ref().unwrap(), "{code}").unwrap();

    process.wait().unwrap();
}
