use subxt::ext::sp_core::{sr25519, Pair};
use crate::AccountId;
use crate::wallet::Signer;

pub type KeypairSignature = sr25519::Signature;

pub fn verify_signature(account_id: &AccountId, signature: &KeypairSignature, message: impl AsRef<[u8]>) -> bool {
    sr25519::Pair::verify(
        signature,
        message,
        &sr25519::Public::from_raw(account_id.0),
    )
}

pub fn sign_message(signer: &Signer, message: impl AsRef<[u8]>) -> KeypairSignature {
    signer.signer().sign(message.as_ref())
}
