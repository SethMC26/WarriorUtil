// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

use warrior_util::utils::base64;

fn main() {
    // encode converts a byte slice to a base64 string
    // use .as_bytes() to convert a string to a byte slice
    let input = "Hello, World!";
    let encoded = base64::encode(input.as_bytes());
    println!("Original: {}", input);
    println!("Encoded:  {}", encoded);

    // decode converts a base64 string back to a byte slice
    // returns a Result since the input may be invalid base64
    let decoded = base64::decode(&encoded).unwrap_or_else(|e| {
        eprintln!("Error decoding: {}", e);
        std::process::exit(1);
    });

    // decoded bytes can be converted back to a string using String::from_utf8
    // note: this will fail if the decoded bytes are not valid UTF-8
    println!("Decoded:  {}", String::from_utf8(decoded).unwrap());

    println!("\n-- Padding examples --");
    // base64 pads output with = to make it a multiple of 4 characters
    // 1 byte  input = 2 base64 chars + == (2 padding chars)
    // 2 byte  input = 3 base64 chars + =  (1 padding char)
    // 3 byte  input = 4 base64 chars      (no padding)
    for s in ["M", "Ma", "Man"] {
        let encoded = base64::encode(s.as_bytes());
        let decoded = base64::decode(&encoded).unwrap();
        println!(
            "{} -> {} -> {}",
            s,
            encoded,
            String::from_utf8(decoded).unwrap()
        );
    }

    println!("\n-- Error handling --");
    // decode returns an Err if the input is not valid base64
    // you can handle this with match or unwrap_or_else
    match base64::decode("invalid!") {
        Ok(bytes) => println!("Decoded: {:?}", bytes),
        Err(e) => println!("Expected error: {}", e),
    }
}
