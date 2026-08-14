use alloy_consensus::transaction::SignerRecoverable;
use alloy_consensus::{Transaction, TxEnvelope};
use alloy_eips::eip2718::Decodable2718;
use alloy_primitives::{Address, B256};

use crate::api::error::ItemError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sender {
    pub address: Address,
    pub transaction_hash: B256,
    pub chain_id: Option<u64>,
}

pub fn recover_sender(raw: &[u8]) -> Result<Sender, ItemError> {
    let mut cursor = raw;
    let envelope =
        TxEnvelope::decode_2718(&mut cursor).map_err(|_| ItemError::MalformedTransaction)?;

    let address = envelope
        .recover_signer()
        .map_err(|_| ItemError::Unrecoverable)?;

    Ok(Sender {
        address,
        transaction_hash: *envelope.tx_hash(),
        chain_id: envelope.chain_id(),
    })
}
