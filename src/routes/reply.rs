use serde::Serialize;
use worker::{Response, Result};

use crate::api::error::RequestError;
use crate::api::response::ErrorBody;

pub fn json<T: Serialize>(value: &T) -> Result<Response> {
    Response::from_json(value)
}

pub fn failed(error: RequestError) -> Result<Response> {
    let body = ErrorBody::from(&error);
    Ok(Response::from_json(&body)?.with_status(error.status()))
}
