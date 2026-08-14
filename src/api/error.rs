use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemError {
    SignatureLength(usize),
    RecoveryId(u8),
    MalformedSignature,
    Unrecoverable,
    MalformedTransaction,
}

impl ItemError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::SignatureLength(_) => "signature_length",
            Self::RecoveryId(_) => "recovery_id",
            Self::MalformedSignature => "malformed_signature",
            Self::Unrecoverable => "unrecoverable",
            Self::MalformedTransaction => "malformed_transaction",
        }
    }
}

impl fmt::Display for ItemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SignatureLength(n) => write!(f, "signature must be 65 bytes, got {n}"),
            Self::RecoveryId(v) => {
                write!(f, "recovery id {v} is not 0, 1, 27, 28 or an EIP-155 value")
            }
            Self::MalformedSignature => write!(f, "signature r or s is not a valid scalar"),
            Self::Unrecoverable => write!(f, "no public key matches this signature and hash"),
            Self::MalformedTransaction => {
                write!(f, "bytes are not an EIP-2718 encoded transaction")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestError {
    BodyTooLarge { bytes: usize, limit: usize },
    BatchTooLarge { items: usize, limit: usize },
    EmptyBatch,
    MalformedJson(String),
    MalformedHex { field: &'static str, index: usize },
    NotFound,
    MethodNotAllowed,
}

impl RequestError {
    pub fn status(&self) -> u16 {
        match self {
            Self::BodyTooLarge { .. } => 413,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            _ => 400,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::BodyTooLarge { .. } => "body_too_large",
            Self::BatchTooLarge { .. } => "batch_too_large",
            Self::EmptyBatch => "empty_batch",
            Self::MalformedJson(_) => "malformed_json",
            Self::MalformedHex { .. } => "malformed_hex",
            Self::NotFound => "not_found",
            Self::MethodNotAllowed => "method_not_allowed",
        }
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BodyTooLarge { bytes, limit } => {
                write!(f, "body is {bytes} bytes, limit is {limit}")
            }
            Self::BatchTooLarge { items, limit } => {
                write!(f, "batch has {items} items, limit is {limit}")
            }
            Self::EmptyBatch => write!(f, "batch must hold at least one item"),
            Self::MalformedJson(why) => write!(f, "body is not valid json: {why}"),
            Self::MalformedHex { field, index } => {
                write!(f, "field {field} of item {index} is not valid hex")
            }
            Self::NotFound => write!(f, "no such endpoint"),
            Self::MethodNotAllowed => write!(f, "this endpoint accepts POST only"),
        }
    }
}
