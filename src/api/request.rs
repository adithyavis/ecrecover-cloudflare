use alloy_primitives::{Address, Bytes, B256};
use serde::Deserialize;

use crate::api::error::{ItemError, RequestError};
use crate::limits::MAX_BATCH_ITEMS;

#[derive(Debug, Deserialize)]
pub struct Batch<T> {
    pub items: Vec<T>,
}

impl<T> Batch<T> {
    pub fn check(&self) -> Result<(), RequestError> {
        if self.items.is_empty() {
            return Err(RequestError::EmptyBatch);
        }
        if self.items.len() > MAX_BATCH_ITEMS {
            return Err(RequestError::BatchTooLarge {
                items: self.items.len(),
                limit: MAX_BATCH_ITEMS,
            });
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct RecoverItem {
    pub hash: B256,
    pub signature: Bytes,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyItem {
    pub message: Option<String>,
    pub message_hex: Option<Bytes>,
    pub signature: Bytes,
    pub address: Option<Address>,
}

impl VerifyItem {
    pub fn payload(&self) -> Result<&[u8], ItemError> {
        match (&self.message, &self.message_hex) {
            (Some(text), None) => Ok(text.as_bytes()),
            (None, Some(raw)) => Ok(raw.as_ref()),
            _ => Err(ItemError::MalformedSignature),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionItem {
    pub raw: Bytes,
}

pub fn parse<T: for<'de> Deserialize<'de>>(body: &str) -> Result<Batch<T>, RequestError> {
    let batch: Batch<T> =
        serde_json::from_str(body).map_err(|e| RequestError::MalformedJson(e.to_string()))?;
    batch.check()?;
    Ok(batch)
}
