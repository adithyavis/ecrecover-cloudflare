use serde::{Deserialize, Serialize};
use worker::{Request, Response, Result};

use crate::api::error::{ItemError, RequestError};
use crate::api::request;
use crate::api::response::{BatchResponse, Outcome};
use crate::limits::MAX_BODY_BYTES;
use crate::routes::reply;

pub async fn run<I, O, F>(mut req: Request, recover: F) -> Result<Response>
where
    I: for<'de> Deserialize<'de>,
    O: Serialize,
    F: Fn(&I) -> std::result::Result<O, ItemError>,
{
    let body = req.text().await?;

    if body.len() > MAX_BODY_BYTES {
        return reply::failed(RequestError::BodyTooLarge {
            bytes: body.len(),
            limit: MAX_BODY_BYTES,
        });
    }

    let batch = match request::parse::<I>(&body) {
        Ok(batch) => batch,
        Err(error) => return reply::failed(error),
    };

    let results = batch
        .items
        .iter()
        .map(|item| Outcome::from_result(recover(item)))
        .collect();

    reply::json(&BatchResponse::new(results))
}
