use alloy_primitives::{eip191_hash_message, Address, B256};

use crate::api::error::ItemError;
use crate::core::signature;

pub fn hash(message: &[u8]) -> B256 {
    eip191_hash_message(message)
}

pub fn recover(message: &[u8], sig: &[u8]) -> Result<Address, ItemError> {
    signature::recover_from_bytes(sig, &hash(message))
}
