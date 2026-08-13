# ecrecover-cloudflare

A Cloudflare Worker that recovers Ethereum addresses from signatures. Written in
Rust, compiled to WebAssembly.

You give it a signature. It tells you which address made that signature. It
holds no keys and stores nothing.

About **3,200 recoveries per second**, roughly 3x faster than the fastest
JavaScript library. Nothing secret ever crosses the network, because a signature
and a message hash are already public.

## Licence

MIT
