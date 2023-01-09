# mempool_rust

⚠️: The library is under development, it is still missing a lot of features that will come soon. ⚠️

[![crates.io](https://img.shields.io/crates/v/mempool_rust.svg)](https://crates.io/crates/mempool_rust)
[![Documentation](https://docs.rs/mempool_rust/badge.svg)](https://docs.rs/mempool_rust)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/mempool_rust.svg)](./LICENSE.txt)
[![CI](https://github.com/0xtlt/mempool_rust/actions/workflows/checks.yml/badge.svg)](https://github.com/0xtlt/mempool_rust/actions/workflows/checks.yml)
[![Issues](https://img.shields.io/github/issues/0xtlt/mempool_rust)](https://img.shields.io/github/issues/0xtlt/mempool_rust)

An ergonomic, [Mempool](https://mempool.space/) API Client for Rust.

- [Changelog](CHANGELOG.md)

## Example

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
mempool_rust = "0.1"
```

And then the code:

```rust,norun
use mempool_rust::MempoolClient;

#[tokio::main]
async fn main() {
    let client = MempoolClient::new(
        "https://mempool.space",
        None,
    )
    .unwrap();

    // OR with tor
    let client = MempoolClient::new(
        "http://url.onion",
        Some("socks5h://127.0.0.1:9050"),
    )
    .unwrap();

    let blocks = client.get_blocks(None).await.unwrap();

    println!("{:#?}", blocks);
}

```

## Key features

- [x] Tor support
- General
  - [ ] GET difficulty adjustment
- Address
  - [ ] GET address
  - [ ] GET address transactions
  - [ ] GET address transactions chain
  - [ ] GET address transactions mempool
  - [ ] GET address UTXO
- Blocks
  - [ ] GET block
  - [ ] GET block header
  - [ ] GET block height
  - [ ] GET block raw
  - [ ] GET block status
  - [ ] GET block tip height
  - [ ] GET block tip hash
  - [ ] GET block transaction ID
  - [ ] GET block transaction IDs
  - [ ] GET block transactions
  - [x] GET blocks
- Mining
  - [ ] GET mining pools
  - [ ] GET mining pool
  - [ ] GET mining pool hashrates
  - [ ] GET mining pool hashrate
  - [ ] GET mining pool blocks
  - [ ] GET hashrate
  - [ ] GET reward stats
  - [ ] GET block fees
  - [ ] GET block rewards
  - [ ] GET block feerates
  - [ ] GET block sizes and weights
- Fees
  - [ ] GET mempool blocks fees
  - [ ] GET recommended fees
- Mempool
  - [ ] GET mempool
  - [ ] GET mempool transaction IDs
  - [ ] GET mempool recent
- Transactions
  - [ ] GET children pay for parent
  - [ ] GET transaction
  - [ ] GET transaction hex
  - [ ] GET transaction merkleblock proof
  - [ ] GET transaction merkle proof
  - [ ] GET transaction outspend
  - [ ] GET transaction outspends
  - [ ] GET transaction raw
  - [ ] GET transaction status
  - [ ] POST transaction

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
