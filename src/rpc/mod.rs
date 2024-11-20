use parity_scale_codec::{Compact, Decode, Error as DecodeError};
use subxt::client::OnlineClientT;
use subxt::{
    ext::scale_encode::EncodeAsFields,
    runtime_api::{RuntimeApi, StaticPayload},
    Error as CallError,
};
use thiserror::Error;

use crate::rpc::types::{DelegateInfo, NeuronInfo, NeuronInfoLite, StakeInfo, SubnetHyperparams, SubnetInfo, SubnetInfov2};
use crate::{
    api::runtime_apis::{
        delegate_info_runtime_api::types::{GetDelegate, GetDelegated, GetDelegates},
        neuron_info_runtime_api::types::{GetNeuron, GetNeuronLite, GetNeurons, GetNeuronsLite},
        stake_info_runtime_api::types::{GetStakeInfoForColdkey, GetStakeInfoForColdkeys},
        subnet_info_runtime_api::types::{
            GetSubnetHyperparams, GetSubnetInfo, GetSubnetInfoV2, GetSubnetsInfo, GetSubnetsInfoV2,
        },
    },
    AccountId,
    SubtensorConfig,
};

pub mod types;

#[derive(Error, Debug)]
pub enum RuntimeApiError {
    #[error(transparent)]
    CallError(#[from] CallError),

    #[error(transparent)]
    DecodeError(#[from] DecodeError),
}

pub async fn call_runtime_api_decoded<T: RuntimeApiPayloadData + EncodeAsFields>(
    runtime_api: &RuntimeApi<SubtensorConfig, impl OnlineClientT<SubtensorConfig>>,
    payload: StaticPayload<T, Vec<u8>>,
) -> Result<<T as RuntimeApiPayloadData>::Response, RuntimeApiError> {
    let result: Vec<u8> = runtime_api.call(payload).await?;

    Ok(<T as RuntimeApiPayloadData>::Response::decode(
        &mut result.as_ref(),
    )?)
}


pub trait RuntimeApiPayloadData {
    type Response: Decode;
}

macro_rules! runtime_api_typing {
    ($data:ty, $response:ty) => {
        impl RuntimeApiPayloadData for $data {
            type Response = $response;
        }
    };
}

runtime_api_typing!(GetDelegate, DelegateInfo);
runtime_api_typing!(GetDelegates, Vec<DelegateInfo>);

runtime_api_typing!(
    GetDelegated,
    Vec<(DelegateInfo, Compact<u64>)>
);

runtime_api_typing!(GetNeurons, Vec<NeuronInfo>);
runtime_api_typing!(GetNeuron, NeuronInfo);
runtime_api_typing!(GetNeuronsLite, Vec<NeuronInfoLite>);
runtime_api_typing!(GetNeuronLite, Vec<NeuronInfoLite>);

runtime_api_typing!(GetStakeInfoForColdkey, Vec<StakeInfo>);

runtime_api_typing!(
    GetStakeInfoForColdkeys,
    Vec<(AccountId, Vec<StakeInfo>)>
);

runtime_api_typing!(GetSubnetHyperparams, Option<SubnetHyperparams>);
runtime_api_typing!(GetSubnetInfo, Option<SubnetInfo>);
runtime_api_typing!(GetSubnetsInfo, Vec<Option<SubnetInfo>>);
runtime_api_typing!(GetSubnetInfoV2, Option<SubnetInfov2>);

runtime_api_typing!(
    GetSubnetsInfoV2,
    Vec<Option<SubnetInfov2>>
);
