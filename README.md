# warrior-util

A small library meant to be used to help Merrimack College students learn and use Rust. This Library is based off of MerrimackUtil.

**Currently project is in an alpha. It is unstable and may change quickly** 
## Purpose

Help Merrimack College students learn and use Rust in their assignments by
providing a handful of commonly‑needed helpers (JSON/serde examples, host
databases, nonce caches, etc.) so they can focus on core learning.

## Getting started

1. **Clone the repo**

   ```bash
   git clone https://github.com/SethMC26/WarriorUtil.git
   cd warrior-util
   ```

2. **Build & run examples**

   ```bash
   cargo build 
   # run hosts database example
   cargo run --example hosts_example
   # run json example
   cargo run --example json_basic
   # run nonce cache example
   cargo run --example multithreaded
   # run cli example 
   cargo run --example cli -- --help
   ```Expected argu

   The examples demonstrate the basic APIs and how serialization/deserialization
   is handled using `serde`.

3. **Use in your project**

   Add to your `Cargo.toml`:

   ```toml
   [dependencies]
   warrior-util = { path = "../path/to/warrior-util" }
   ```

   and import from the crate (`use warrior_util::…`) as needed.

## Features

This library provides the following utilities:

- **Base64 Encoding/Decoding**: RFC 4648 compliant encoding and decoding
- **Command Line Interface (CLI) Parsing**: Flexible option parsing with short (-p) and long (--port) forms, argument validation, help text generation, and easy argument lookup.
- **Nonce Cache**: Thread-safe caching for nonces(number once) with configurable expiry, and nonce size. 
- **Time Utilities**: Basic time utility similiar to in java `getCurrentTimeMillis()`
- **Network Hosts Database**: JSON-based host database compatible with MerrimackUtil format. Used for networking/Network security projects at Merrimack College.
- **Skip List**: Probabilistic ordered data structure with fast search, insert, and delete. As shown from Randomized Algorithms 

```rust
// Base64 usage
use warrior_util::utils::base64::{encode, decode};
let encoded = encode(b"Hello, World!");
let decoded = decode(&encoded).unwrap();

// CLI parsing usage
use warrior_util::utils::cli::{LongOp, get_op_map};
let ops = vec![LongOp::new("p", "port", "Port to use")];
let op_map = get_op_map(&ops).unwrap();
let port = op_map.get(ops[0]).unwrap();

// Nonce cache usage
use warrior_util::utils::nonce_cache::NonceCache;
let cache = NonceCache::new(16, 30000);
let nonce = cache.get_nonce().unwrap();

// Time utilities usage
use warrior_util::utils::time::current_time_millis;
let timestamp = current_time_millis().unwrap();

// Network hosts database usage
use warrior_util::net::net_db::HostsDatabase;
let json = r#"{"hosts": [{"host-name": "example.com", "address": "192.168.1.1", "port": 8080}]}"#;
let db = HostsDatabase::from_json_str(json).unwrap();

//skip_lists
let list = skip_list![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let exists = list.exists(&value);
list.delete(&2);
list.insert(11);

```

## 📁 Project structure
- `examples/` – runnable demos (hosts database, JSON/serde usage, nonce cache, skip list)  
    - `hosts_database` - Examples for the Host Database
    - `json` - Examples of JSON
    - `nonce_cache` - Examples of nonce cache
    - `base64` - Example of base64 decoder/encoded
    - `collections` - Examples for skip list and future data structures
- `src/` – core library code  
    - `net/` - Network Database(hosts database)
    - `utils/` – nonce_cache, errors(for Utils module), time, cli, Base64 encoder/decoder
    - `collections/` - Skip Lists
    - `lib.rs` Library exports
- `Cargo.toml` – crate manifest

## Roadmap
- Random object
    - Create wrapper for secure, and deterministic PRGs for use in projects
- Crypto
    - Update with a Good Crypto crate, add examples and provide custom features to make working with crypto primitives easier
    - We really need a good crypto crate
- Nonce cache
    - Fix current hacky solution with a good secure Random 
- Logging feature
- Fun data structures like ~~skip lists~~ and bloom filters
- JSON
    - Custom traits, structs and marcos to help serde_json behave more like MerrimackUtil JSON
- Examples
    - Add networking examples
    - Add Crypto Examples
    - Other examples from merrimack_cs_examples
- Reduce dependencies as much as possible
    - Serde_json and serde are massive dependencies
        - Look into custom solutions or only using custom crate/cargo imports to reduce dependency footprint

## Contributions
Contributions are welcome! Please contact for further information on contributions: holtzmans@merrimack.edu
# Author
Seth Holtzman 