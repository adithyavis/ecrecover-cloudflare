use std::time::Instant;

use alloy_primitives::hex;
use alloy_signer::SignerSync;
use alloy_signer_local::PrivateKeySigner;
use ecrecover_cloudflare::api::request::{Batch, RecoverItem};
use ecrecover_cloudflare::core::{message, signature};

const KEY: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const ROUNDS: usize = 2000;

fn main() {
    let signer: PrivateKeySigner = KEY.parse().unwrap();
    let payload = b"benchmark payload";
    let hash = message::hash(payload);
    let sig = signer.sign_message_sync(payload).unwrap().as_bytes();

    let start = Instant::now();
    for _ in 0..ROUNDS {
        signature::recover_from_bytes(&sig, &hash).unwrap();
    }
    let crypto = start.elapsed().as_secs_f64() / ROUNDS as f64 * 1000.0;

    let item = format!(
        "{{\"hash\": \"{}\", \"signature\": \"0x{}\"}}",
        hash,
        hex::encode(sig)
    );
    let body = format!("{{\"items\":[{}]}}", vec![item; ROUNDS].join(","));

    let start = Instant::now();
    let batch: Batch<RecoverItem> = serde_json::from_str(&body).unwrap();
    let parse = start.elapsed().as_secs_f64() / ROUNDS as f64 * 1000.0;

    let start = Instant::now();
    for entry in &batch.items {
        signature::recover_from_bytes(entry.signature.as_ref(), &entry.hash).unwrap();
    }
    let recover = start.elapsed().as_secs_f64() / ROUNDS as f64 * 1000.0;

    println!("native, per item, {ROUNDS} rounds:");
    println!(
        "  recover only     {crypto:.4} ms  ({:.0}/sec)",
        1000.0 / crypto
    );
    println!("  json parse       {parse:.4} ms");
    println!("  parse + recover  {:.4} ms", parse + recover);
}
