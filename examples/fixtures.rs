use alloy_primitives::hex;
use alloy_signer::SignerSync;
use alloy_signer_local::PrivateKeySigner;
use ecrecover_cloudflare::core::message;
use serde_json::json;

const KEY: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

const MESSAGES: [&str; 3] = [
    "example.com wants you to sign in with your Ethereum account",
    "hello world",
    "",
];

fn main() {
    let signer: PrivateKeySigner = KEY.parse().unwrap();
    let address = signer.address();

    let mut verify = Vec::new();
    let mut recover = Vec::new();

    for text in MESSAGES {
        let payload = text.as_bytes();
        let sig = format!(
            "0x{}",
            hex::encode(signer.sign_message_sync(payload).unwrap().as_bytes())
        );

        verify.push(json!({
            "message": text,
            "signature": sig,
            "address": address,
        }));

        recover.push(json!({
            "hash": message::hash(payload),
            "signature": sig,
        }));
    }

    let out = json!({
        "verify": { "items": verify },
        "recover": { "items": recover },
    });

    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
