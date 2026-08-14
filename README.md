# ecrecover-cloudflare

A Cloudflare Worker that recovers Ethereum addresses from signatures. Written in
Rust, compiled to WebAssembly.

You give it a signature. It tells you which address made that signature. It
holds no keys and stores nothing.

About **3,200 recoveries per second**, roughly 3x faster than the fastest
JavaScript library.

## Setup

You need Rust (recent stable), the WebAssembly target, and Node.

```sh
rustup update stable
rustup target add wasm32-unknown-unknown
cargo install worker-build
npm install
```

Full walkthrough for a machine with no Rust on it:
[specs/rust-setup.md](specs/rust-setup.md).

## Run it

```sh
npm run dev
```

Check it is alive:

```sh
curl localhost:8787/
```

## API

Three endpoints, all taking a batch:

```json
{ "items": [ ... ] }
```

Every response has the same shape. One result per item, in order, plus tallies:

```json
{ "results": [ ... ], "recovered": 2, "failed": 1 }
```

### POST /recover

```sh
curl -X POST localhost:8787/recover -H 'content-type: application/json' -d '{
  "items": [
    {
      "hash": "0x2105e790350adafe6bd51b99da4362fb0dbbc9a8835f535073029df0c68a59ec",
      "signature": "0x8f795fb83f6c92ec7f8fd1294466d10765110c39bcf4435cbc3752aa407370072061cf2051f22d43a2a104d339b54f3a6681899d76ea0386b030c247caa1263d1c"
    }
  ]
}'
```

Use this when you already have the digest, for example an EIP-712 hash.

### POST /verify

Send `message` (plain text) **or** `messageHex` (`0x...`), never both. The
service applies the EIP-191 prefix for you.

Add `address` and you get a `matches` field back. Leave it out and you just get
the recovered address.

```sh
curl -X POST localhost:8787/verify -H 'content-type: application/json' -d '{
  "items": [
    {
      "message": "example.com wants you to sign in with your Ethereum account",
      "signature": "0x8f795fb83f6c92ec7f8fd1294466d10765110c39bcf4435cbc3752aa407370072061cf2051f22d43a2a104d339b54f3a6681899d76ea0386b030c247caa1263d1c",
      "address": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
    }
  ]
}'
```

### POST /transaction

```sh
curl -X POST localhost:8787/transaction -H 'content-type: application/json' -d '{
  "items": [ { "raw": "0xf901ad830773a2841dcd650083034ff094..." } ]
}'
```

Handles legacy, EIP-2930, EIP-1559, EIP-4844 and EIP-7702 transactions.
`chainId` is absent for pre-EIP-155 transactions.

### Generating test requests

`examples/fixtures.rs` signs some messages with a well-known test key and prints
ready-to-post bodies for `/recover` and `/verify`:

```sh
cargo run --example fixtures
```

## Errors

A bad **item** returns `200`, with that item marked failed:

```json
{"ok":false,"error":{"code":"signature_length","message":"signature must be 65 bytes, got 2"}}
```

## Development

| Command | What it does |
|---|---|
| `cargo check` | Type-checks. Fastest. |
| `cargo test` | Runs the tests against real signatures and mainnet transactions. |
| `cargo clippy` | Suggests better Rust. |
| `cargo fmt` | Formats. |
| `cargo run --release --example bench` | Measures recoveries per second. |
| `npm run dev` | Builds to WebAssembly and serves locally. |
| `npm run deploy` | Ships it to Cloudflare. |

## Deploy

```sh
npx wrangler login
npm run deploy
```

The worker name comes from [wrangler.toml](wrangler.toml). It needs no
bindings, no secrets, and no environment variables.

## Licence

MIT
