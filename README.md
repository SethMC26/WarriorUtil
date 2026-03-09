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

## 📁 Project structure
- `examples/` – runnable demos (hosts database, JSON/serde usage, nonce cache)  
    - `hosts_database` - Examples for the Host Database
    - `json` - Examples of JSON
    - `nonce_cache` - Examples of nonce cache
- `src/` – core library code  
    - `net/` - Network Database(hosts database)
    - `utils/` – nonce_cache, errors(for Utils module), time, cli, Base64 encoder/decoder
    - `lib.rs` Library exports
- `Cargo.toml` – crate manifest

## Roadmap
- JSON
    - Custom traits, structs and marcos to help serde_json behave more like MerrimackUtil JSON
- Crypto
    - Update with a Good Crypto crate, add examples and provide custom features to make working with crypto primitives easier
- Nonce cache
    - Fix current hacky solution with a good secure Random 
- Logging feature
- Fun data structures like skip lists and bloom filters
- Add testing to ensure library stability
- CI/CD?
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