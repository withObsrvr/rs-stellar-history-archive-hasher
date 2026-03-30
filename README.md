# stellar-history-archive-hasher

WASM library for hashing Stellar transaction history archive entries. Provides functions to hash `TransactionHistoryEntry` and `TransactionHistoryResultEntry` XDR data.

## Install

**npm:**
```bash
npm install @withobsrvr/stellar-history-archive-hasher
```

**Cargo:**
```bash
cargo add stellar-history-archive-hasher
```

## Usage

### Node.js (CommonJS)
```js
const {
  hash_transaction_history_entry,
  hash_transaction_history_result_entry
} = require('@withobsrvr/stellar-history-archive-hasher');

const xdrBytes = new Uint8Array(Buffer.from(base64String, 'base64'));
const hash = hash_transaction_history_entry(xdrBytes);
```

### Rust
```rust
use stellar_history_archive_hasher::internal::{
    hash_transaction_history_entry,
    hash_transaction_history_result_entry,
};

let hash = hash_transaction_history_entry(&xdr_bytes).unwrap();
```

## Build

### Prerequisites

- Rust toolchain (`rustup`)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Build for Node.js (CommonJS)
```bash
wasm-pack build --target nodejs --scope withobsrvr
```

### Build for bundlers (ES modules)
```bash
wasm-pack build --target bundler --scope withobsrvr
```

Output goes to the `pkg/` directory.

## Test

```bash
# Rust unit tests
cargo test

# Manual smoke test (requires Node.js build)
node index.js
```

## Upgrade dependencies

```bash
rustup update
cargo update
```

## Publish

```bash
# npm (requires npm login to @withobsrvr org)
wasm-pack build --target nodejs --scope withobsrvr
wasm-pack publish --access public

# crates.io
cargo publish
```
