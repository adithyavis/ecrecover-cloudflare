use worker::{Request, Response, Result, RouteContext};

use crate::api::request::VerifyItem;
use crate::api::response::Verified;
use crate::core::message;
use crate::routes::batch;

pub async fn handle(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    batch::run(req, |item: &VerifyItem| {
        let address = message::recover(item.payload()?, item.signature.as_ref())?;

        Ok(Verified {
            address,
            matches: item.address.map(|claimed| claimed == address),
        })
    })
    .await
}
