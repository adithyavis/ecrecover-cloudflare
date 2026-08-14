use alloy_primitives::{Address, B256};
use serde::Serialize;

use crate::api::error::{ItemError, RequestError};

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: &'static str,
    pub message: String,
}

impl From<&ItemError> for ErrorBody {
    fn from(error: &ItemError) -> Self {
        Self {
            code: error.code(),
            message: error.to_string(),
        }
    }
}

impl From<&RequestError> for ErrorBody {
    fn from(error: &RequestError) -> Self {
        Self {
            code: error.code(),
            message: error.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Failure {
    pub ok: bool,
    pub error: ErrorBody,
}

#[derive(Debug, Serialize)]
pub struct Success<T> {
    pub ok: bool,
    #[serde(flatten)]
    pub value: T,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Outcome<T> {
    Success(Success<T>),
    Failure(Failure),
}

impl<T> Outcome<T> {
    pub fn from_result(result: Result<T, ItemError>) -> Self {
        match result {
            Ok(value) => Self::Success(Success { ok: true, value }),
            Err(error) => Self::Failure(Failure {
                ok: false,
                error: ErrorBody::from(&error),
            }),
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(_))
    }
}

#[derive(Debug, Serialize)]
pub struct BatchResponse<T> {
    pub results: Vec<Outcome<T>>,
    pub recovered: usize,
    pub failed: usize,
}

impl<T> BatchResponse<T> {
    pub fn new(results: Vec<Outcome<T>>) -> Self {
        let recovered = results.iter().filter(|r| r.is_success()).count();
        let failed = results.len() - recovered;
        Self {
            results,
            recovered,
            failed,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Recovered {
    pub address: Address,
}

#[derive(Debug, Serialize)]
pub struct Verified {
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SenderFound {
    pub address: Address,
    pub transaction_hash: B256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<u64>,
}
