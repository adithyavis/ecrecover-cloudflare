use worker::{Request, Response, Result, RouteContext};

use crate::api::request::TransactionItem;
use crate::api::response::SenderFound;
use crate::core::transaction;
use crate::routes::batch;

pub async fn handle(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    batch::run(req, |item: &TransactionItem| {
        transaction::recover_sender(item.raw.as_ref()).map(|found| SenderFound {
            address: found.address,
            transaction_hash: found.transaction_hash,
            chain_id: found.chain_id,
        })
    })
    .await
}
