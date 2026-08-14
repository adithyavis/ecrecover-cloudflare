use alloy_primitives::{address, hex, keccak256, Address, B256};
use ecrecover_cloudflare::api::error::ItemError;
use ecrecover_cloudflare::core::{message, signature, transaction};

const LEGACY_RAW: &str = "f901ad830773a2841dcd650083034ff0943a5cc8689d1b0cef2c317bc5c0ad6ce88b27d59780b901440dcd7a6c000000000000000000000000e4f09f086837270623e4ce4f4d101f008c4485d3000000000000000000000000000000000000000000000000000000000ad5b981000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48000000000000000000000000000000000000000000000000000000006a7f2f90000000000000000000000000000000000000000000000000000000000003417600000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000004157d65c7ffbb3528d17f0d02c029577c2ae9e910ac46b64cd9685e4eb6be5f3f36c65a270d036e653d7c4a6dc51af7ff6dd6c3e4f6ad601611430cc652be2ab891b0000000000000000000000000000000000000000000000000000000000000025a0d2f4bfd3ef128f6ff71bf89614ee454d8dc7d6ce58928276a930a63eb9c5cca9a06a2dd776c4e726156297454328dba60e0da47592f77bcc2e2505ec015e21cd66";

const EIP1559_RAW: &str = "02f9012e0182272480840d62876e830493e094da7afeed01fe625cf15d187a19f94b45f00b8c5f80b8c4a9114b0f00000000000000000000000090f73fea1ee2dc514d4dbac0bff7ff04b933767f8572bdba86a20264869c9ba9a80466276690132a91547b719314a1ebea354863000000000000000000000000000000000000000000000000000000006a7dde17000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000005f72efcc080a07059d1a19fd3403cc92721cbd0e61cd3bf69948d0c0b84e0d9546dac444697aaa07a83ee0b7006069d14844ea1e36953bc3d8665272b10e3a9141b808543e9257d";

#[test]
fn recovers_the_sender_of_a_real_legacy_transaction() {
    let raw = hex::decode(LEGACY_RAW).unwrap();
    let found = transaction::recover_sender(&raw).unwrap();

    assert_eq!(
        found.address,
        address!("b02c6c40a798184d5e012fbb1dc698977671f8fc")
    );
    assert_eq!(
        found.transaction_hash,
        "0xeec6abae93f60a9304674159d0ba7461ad3d1c3ed09ec09a16903e8d43d6eceb"
            .parse::<B256>()
            .unwrap()
    );
    assert_eq!(found.chain_id, Some(1));
}

#[test]
fn recovers_the_sender_of_a_real_eip1559_transaction() {
    let raw = hex::decode(EIP1559_RAW).unwrap();
    let found = transaction::recover_sender(&raw).unwrap();

    assert_eq!(
        found.address,
        address!("fcb4150a75485727203ac1f18a78bbb929175b03")
    );
    assert_eq!(
        found.transaction_hash,
        "0xac00b0206de51dca92e7f026e26be4ace8783770e63c671dca817f61a15de5b7"
            .parse::<B256>()
            .unwrap()
    );
    assert_eq!(found.chain_id, Some(1));
}

#[test]
fn rejects_bytes_that_are_not_a_transaction() {
    let error = transaction::recover_sender(&[0x01, 0x02, 0x03]).unwrap_err();
    assert_eq!(error, ItemError::MalformedTransaction);
}

#[test]
fn accepts_every_encoding_of_the_recovery_id() {
    let body = [0x11u8; 64];

    for v in [0u8, 1, 27, 28, 37, 38] {
        let mut raw = [0u8; 65];
        raw[..64].copy_from_slice(&body);
        raw[64] = v;
        assert!(signature::parse(&raw).is_ok(), "v={v} should parse");
    }

    let mut bad = [0u8; 65];
    bad[..64].copy_from_slice(&body);
    bad[64] = 5;
    assert_eq!(
        signature::parse(&bad).unwrap_err(),
        ItemError::RecoveryId(5)
    );
}

#[test]
fn rejects_a_signature_of_the_wrong_length() {
    assert_eq!(
        signature::parse(&[0u8; 64]).unwrap_err(),
        ItemError::SignatureLength(64)
    );
}

#[test]
fn hashes_a_message_the_eip191_way() {
    let payload = b"Hello World";

    let mut prefixed = format!("\x19Ethereum Signed Message:\n{}", payload.len()).into_bytes();
    prefixed.extend_from_slice(payload);

    assert_eq!(message::hash(payload), keccak256(prefixed));
}

#[test]
fn round_trips_a_message_signature() {
    use alloy_signer::SignerSync;
    use alloy_signer_local::PrivateKeySigner;

    let signer: PrivateKeySigner =
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
            .parse()
            .unwrap();

    let expected: Address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    assert_eq!(signer.address(), expected);

    let payload = b"sign in to ecrecover-cloudflare";
    let sig = signer.sign_message_sync(payload).unwrap();

    let recovered = message::recover(payload, &sig.as_bytes()).unwrap();
    assert_eq!(recovered, expected);
}
