use anyhow::{Context, Ok, Result};
use crypto_hash::{hex_digest, Algorithm};
use hex::ToHex;
use pgp::composed::message::Message;
use pgp::types::public;
use pgp::{composed, composed::signed_key::*, crypto, types::SecretKeyTrait, Deserializable};
use rand::prelude::*;
use smallvec::*;

pub fn verify_signature(message: String, public_key: String) -> Result<bool> {
    let msg = Message::from_string(&message).context("invalid message")?.0;
    let public_key = SignedPublicKey::from_string(&public_key)
        .context("invalid public key")?
        .0;
    let verified = msg.verify(&public_key);
    Ok(verified.is_ok())
}
