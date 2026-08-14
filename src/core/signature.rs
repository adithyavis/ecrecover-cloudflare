use alloy_primitives::{Address, Signature, B256};

use crate::api::error::ItemError;

pub const SIGNATURE_BYTES: usize = 65;

pub fn parse(bytes: &[u8]) -> Result<Signature, ItemError> {
    if bytes.len() != SIGNATURE_BYTES {
        return Err(ItemError::SignatureLength(bytes.len()));
    }

    let mut raw = [0u8; SIGNATURE_BYTES];
    raw.copy_from_slice(bytes);
    raw[64] = parity(raw[64])?;

    Signature::from_raw(&raw).map_err(|_| ItemError::MalformedSignature)
}

fn parity(v: u8) -> Result<u8, ItemError> {
    match v {
        0 | 1 => Ok(v),
        27 | 28 => Ok(v - 27),
        v if v >= 35 => Ok((v - 35) % 2),
        v => Err(ItemError::RecoveryId(v)),
    }
}

pub fn recover(signature: &Signature, prehash: &B256) -> Result<Address, ItemError> {
    signature
        .recover_address_from_prehash(prehash)
        .map_err(|_| ItemError::Unrecoverable)
}

pub fn recover_from_bytes(signature: &[u8], prehash: &B256) -> Result<Address, ItemError> {
    recover(&parse(signature)?, prehash)
}
