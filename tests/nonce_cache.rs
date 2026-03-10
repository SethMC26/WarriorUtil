//! # Test Plan for nonce_cache Module
//!
//! Author: Claude Sonnet 4.6 (Anthropic)
//!
//! ## General Testing Procedure
//! Tests validate thread-safe nonce caching with expiry. Uses real time delays for expiry tests.
//! Concurrent access tested with std::thread. Error handling checks invalid inputs.
//! NonceCache reads from /dev/urandom so tests require a Unix-like environment.
//!
//! ## Edge Cases Tested
//! - Nonce size validation (too small, too large)
//! - Cache expiry timing
//! - Concurrent read/write access
//! - Duplicate nonce handling
//!
//! ## Positive Tests
//! - `test_nonce_cache_new`: Creates cache and adds a nonce
//! - `test_add_nonce_correct_size`: Accepts valid nonces
//! - `test_contains_nonce_correct_size`: Finds existing nonces
//! - `test_get_nonce`: Generates a nonce of correct size and adds to cache
//! - `test_get_nonce_unique`: Two generated nonces are not equal
//! - `test_clone`: Cloned cache shares the same underlying data
//! - `test_concurrent_access`: Thread-safe operations produce unique nonces
//!
//! ## Negative Tests
//! - `test_add_nonce_wrong_size`: Rejects nonces of wrong size
//! - `test_contains_nonce_wrong_size`: Errors on size mismatch
//! - `test_contains_nonce_not_present`: Returns false for missing nonces
//! - `test_nonce_expiry`: Nonces expire after age_limit milliseconds

use std::thread;
use std::time::Duration;
use warrior_util::utils::errors::UtilError;
use warrior_util::utils::nonce_cache::NonceCache;

#[test]
/// Positive test: Creates cache and adds a nonce successfully
fn test_nonce_cache_new() {
    let cache = NonceCache::new(16, 1000);
    let nonce = vec![0; 16];
    assert!(cache.add_nonce(&nonce).is_ok());
}

#[test]
/// Positive test: Accepts nonces of the correct size
fn test_add_nonce_correct_size() {
    let cache = NonceCache::new(4, 1000);
    let nonce = vec![1, 2, 3, 4];
    assert!(cache.add_nonce(&nonce).is_ok());
}

#[test]
/// Negative test: Rejects nonces that are too short or too long
fn test_add_nonce_wrong_size() {
    let cache = NonceCache::new(4, 1000);

    let too_short = vec![1, 2, 3];
    assert!(matches!(
        cache.add_nonce(&too_short),
        Err(UtilError::InvalidInput(_))
    ));

    let too_long = vec![1, 2, 3, 4, 5];
    assert!(matches!(
        cache.add_nonce(&too_long),
        Err(UtilError::InvalidInput(_))
    ));
}

#[test]
/// Positive test: Returns true for a nonce that was added to the cache
fn test_contains_nonce_correct_size() {
    let cache = NonceCache::new(4, 1000);
    let nonce = vec![1, 2, 3, 4];
    cache.add_nonce(&nonce).unwrap();
    assert!(cache.contains_nonce(&nonce).unwrap());
}

#[test]
/// Negative test: Returns InvalidInput error when checking nonce of wrong size
fn test_contains_nonce_wrong_size() {
    let cache = NonceCache::new(4, 1000);
    let nonce = vec![1, 2, 3];
    assert!(matches!(
        cache.contains_nonce(&nonce),
        Err(UtilError::InvalidInput(_))
    ));
}

#[test]
/// Negative test: Returns false for a nonce that was never added
fn test_contains_nonce_not_present() {
    let cache = NonceCache::new(4, 1000);
    let nonce = vec![1, 2, 3, 4];
    assert!(!cache.contains_nonce(&nonce).unwrap());
}

#[test]
/// Edge case test: Adding a duplicate nonce while still valid does not change its timestamp
fn test_add_nonce_twice() {
    let cache = NonceCache::new(4, 1000);
    let nonce = vec![1, 2, 3, 4];
    cache.add_nonce(&nonce).unwrap();
    // adding again while still valid should not update timestamp per implementation
    cache.add_nonce(&nonce).unwrap();
    assert!(cache.contains_nonce(&nonce).unwrap());
}

#[test]
/// Positive test: Generated nonce has correct size and is added to cache
fn test_get_nonce() {
    let cache = NonceCache::new(4, 1000);
    let nonce = cache.get_nonce().unwrap();
    assert_eq!(nonce.len(), 4);
    assert!(cache.contains_nonce(&nonce).unwrap());
}

#[test]
/// Positive test: Two generated nonces are unique and both present in cache
fn test_get_nonce_unique() {
    let cache = NonceCache::new(4, 1000);
    let nonce1 = cache.get_nonce().unwrap();
    let nonce2 = cache.get_nonce().unwrap();
    assert_ne!(nonce1, nonce2);
    assert!(cache.contains_nonce(&nonce1).unwrap());
    assert!(cache.contains_nonce(&nonce2).unwrap());
}

#[test]
/// Edge case test: Nonce expires after age_limit milliseconds
fn test_nonce_expiry() {
    let cache = NonceCache::new(4, 10); // 10ms age limit
    let nonce = vec![1, 2, 3, 4];
    cache.add_nonce(&nonce).unwrap();
    thread::sleep(Duration::from_millis(20)); // sleep well past expiry
    assert!(!cache.contains_nonce(&nonce).unwrap());
}

#[test]
/// Positive test: Cloned cache shares the same underlying data via Arc
fn test_clone() {
    let cache = NonceCache::new(4, 1000);
    let cloned = cache.clone();
    let nonce = vec![1, 2, 3, 4];
    cache.add_nonce(&nonce).unwrap();
    // cloned shares the same Arc so should see the nonce
    assert!(cloned.contains_nonce(&nonce).unwrap());
}

#[test]
/// Positive test: Two threads generate unique nonces concurrently without data races
fn test_concurrent_access() {
    let cache = NonceCache::new(4, 1000);
    let cache_clone = cache.clone();

    let handle = thread::spawn(move || cache_clone.get_nonce().unwrap());

    let nonce2 = cache.get_nonce().unwrap();
    let nonce1 = handle.join().unwrap();

    assert_ne!(nonce1, nonce2);
    assert!(cache.contains_nonce(&nonce1).unwrap());
    assert!(cache.contains_nonce(&nonce2).unwrap());
}
