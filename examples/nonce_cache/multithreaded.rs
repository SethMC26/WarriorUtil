// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

// examples/multithreaded.rs
use std::thread;
use warrior_util::utils::nonce_cache::NonceCache;

fn main() {
    // no Arc needed — NonceCache handles sharing internally
    let cache = NonceCache::new(8, 60_000);

    let mut handles = vec![];

    // spawn 5 producer threads — each generates nonces
    for thread_id in 0..5 {
        let cache = cache.clone(); // cheap — just increments Arc ref count

        let handle = thread::spawn(move || {
            println!("thread {} — generating nonces", thread_id);

            for i in 0..3 {
                match cache.get_nonce() {
                    Ok(nonce) => println!("thread {} nonce {}: {:?}", thread_id, i, nonce),
                    Err(e) => eprintln!("thread {} failed to get nonce: {}", thread_id, e),
                }
            }
        });

        handles.push(handle);
    }

    // spawn 3 consumer threads — each checks if a nonce exists
    for thread_id in 0..3 {
        let cache = cache.clone();

        let handle = thread::spawn(move || {
            let nonce = match cache.get_nonce() {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("consumer thread {} failed: {}", thread_id, e);
                    return;
                }
            };

            match cache.contains_nonce(&nonce) {
                Ok(true) => println!("consumer thread {} — nonce found ✅", thread_id),
                Ok(false) => println!("consumer thread {} — nonce NOT found ❌", thread_id),
                Err(e) => eprintln!("consumer thread {} — error: {}", thread_id, e),
            }
        });

        handles.push(handle);
    }

    // replay attack simulation
    {
        let cache = cache.clone();

        let handle = thread::spawn(move || {
            println!("\n--- replay attack simulation ---");

            let nonce = match cache.get_nonce() {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("replay thread failed: {}", e);
                    return;
                }
            };

            // nonce should exist — was just added by get_nonce
            match cache.contains_nonce(&nonce) {
                Ok(true) => println!("replay: nonce exists in cache ✅"),
                Ok(false) => println!("replay: nonce missing — unexpected ❌"),
                Err(e) => eprintln!("replay: error: {}", e),
            }

            // try to add same nonce again — should not update timestamp
            match cache.add_nonce(&nonce) {
                Ok(()) => println!("replay: add_nonce called again — nonce unchanged ✅"),
                Err(e) => eprintln!("replay: error: {}", e),
            }

            // confirm nonce still valid
            match cache.contains_nonce(&nonce) {
                Ok(true) => println!("replay: nonce still valid ✅"),
                Ok(false) => println!("replay: nonce expired — unexpected ❌"),
                Err(e) => eprintln!("replay: error: {}", e),
            }
        });

        handles.push(handle);
    }

    // wait for all threads to finish
    for handle in handles {
        handle.join().expect("thread panicked");
    }

    println!("\n--- all threads complete ---");
}
