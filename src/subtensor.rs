use subxt::{Error, OnlineClient};

use crate::SubtensorConfig;

pub type Subtensor = OnlineClient<SubtensorConfig>;

pub enum SubtensorUrl {
    Finney,
    Archive,
    Test,
    Local,
}

impl AsRef<str> for SubtensorUrl {
    fn as_ref(&self) -> &str {
        match self {
            SubtensorUrl::Finney => "wss://entrypoint-finney.opentensor.ai:443",
            SubtensorUrl::Archive => "wss://archive.chain.opentensor.ai:443/",
            SubtensorUrl::Test => "wss://test.finney.opentensor.ai:443/",
            SubtensorUrl::Local => "ws://127.0.0.1:9944",
        }
    }
}

pub async fn from_url(url: impl AsRef<str>) -> Result<Subtensor, Error> {
    Subtensor::from_url(url).await
}
