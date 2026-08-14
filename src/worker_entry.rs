use worker::{event, Env, Request, Response, Result, Router};

use crate::limits::{MAX_BATCH_ITEMS, MAX_BODY_BYTES};
use crate::routes;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    Router::new()
        .get("/", |_, _| {
            Response::from_json(&serde_json::json!({
                "service": "ecrecover-cloudflare",
                "endpoints": ["/recover", "/verify", "/transaction"],
                "maxBatchItems": MAX_BATCH_ITEMS,
                "maxBodyBytes": MAX_BODY_BYTES,
            }))
        })
        .post_async("/recover", routes::recover::handle)
        .post_async("/verify", routes::verify::handle)
        .post_async("/transaction", routes::transaction::handle)
        .run(req, env)
        .await
}
