use std::ops::{Div, Mul};
use subxt::tx::Payload;
use crate::api;

#[derive(Debug)]
pub struct NormalizedWeight {
    pub uid: u16,
    pub weight: u16,
}

pub fn normalize_weights<T>(
    weights: impl IntoIterator<Item = T> + Clone,
) -> Option<impl Iterator<Item = u16>>
where
    T: Mul<u16> + Ord + Clone,
    <T as Mul<u16>>::Output: Div<T>,
    u16: From<<<T as Mul<u16>>::Output as Div<T>>::Output>,
{
    let max_weight = weights.clone().into_iter().max()?;

    Some(
        weights
            .into_iter()
            .map(move |weight| u16::from((weight * u16::MAX) / max_weight.clone())),
    )
}

pub fn set_weights_payload(
    netuid: u16,
    weights: Vec<NormalizedWeight>,
    version_key: u64,
) -> impl Payload {
    let (uids, weight_values): (Vec<_>, Vec<_>) =
        weights.into_iter().map(|w| (w.uid, w.weight)).unzip();

    api::tx()
        .subtensor_module()
        .set_weights(netuid, uids, weight_values, version_key)
}
