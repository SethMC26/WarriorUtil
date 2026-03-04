use base64::{Engine, engine::general_purpose::STANDARD};
use warrior_util::NonceCache;
fn main() {
    let nonce_cache = NonceCache::new(4, 60);
    let nonce = nonce_cache.get_nonce().unwrap();

    // base64 encode the nonce for printing
    let nonce_base64 = STANDARD.encode(&nonce);
    println!("Nonce (base64): {}", nonce_base64);
}
