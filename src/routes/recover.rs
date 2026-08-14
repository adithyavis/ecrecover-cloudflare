use worker::{Request, Response, Result, RouteContext};

use crate::api::request::RecoverItem;
use crate::api::response::Recovered;
use crate::core::signature;
use crate::routes::batch;

pub async fn handle(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    batch::run(req, |item: &RecoverItem| {
        signature::recover_from_bytes(item.signature.as_ref(), &item.hash)
            .map(|address| Recovered { address })
    })
    .await
}
