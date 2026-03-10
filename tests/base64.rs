// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

//! # Test Plan for base64 Module
//!
//! Author: Claude Sonnet 4.6 (Anthropic)
//!
//! ## General Testing Procedure
//! Integration tests validate Base64 encoding and decoding functions against RFC 4648 standard.
//! Tests use byte slices of various lengths and verify output strings match expected Base64.
//! Decoding tests check error handling for invalid inputs.
//!
//! ## Edge Cases Tested
//! - Empty input (0 bytes)
//! - Single byte (1 byte, requires 2 padding chars)
//! - Two bytes (2 bytes, requires 1 padding char)
//! - Three bytes (3 bytes, no padding)
//! - Invalid input lengths (not multiple of 4)
//! - Invalid characters in input string
//! - Non-ASCII characters (>127)
//!
//! ## Positive Tests
//! - `test_encode_*`: Various input sizes produce correct Base64 strings
//! - `test_decode_valid`: Decodes standard Base64 strings correctly
//! - `test_encode_decode_roundtrip`: Ensures encode/decode is reversible
//! - `test_decode_with_one_padding`: Handles input with 1 padding char
//! - `test_decode_with_two_padding`: Handles input with 2 padding chars
//!
//! ## Negative Tests
//! - `test_decode_invalid_length`: Rejects strings not multiple of 4
//! - `test_decode_invalid_char`: Fails on non-Base64 characters
//! - `test_decode_non_ascii`: Rejects non-ASCII input

use warrior_util::utils::base64::{decode, encode};

// "Hack the Planet!" - Hackers (1995)
const HACK_THE_PLANET: &[u8] = b"Hack the Planet!";
const HACK_THE_PLANET_ENCODED: &str = "SGFjayB0aGUgUGxhbmV0IQ==";

#[test]
/// Positive test: Encodes empty input
fn test_encode_empty() {
    assert_eq!(encode(&[]), "");
}

#[test]
/// Positive test: Encodes 1 byte with 2 padding chars
fn test_encode_one_byte() {
    assert_eq!(encode(b"f"), "Zg==");
}

#[test]
/// Positive test: Encodes 2 bytes with 1 padding char
fn test_encode_two_bytes() {
    assert_eq!(encode(b"fo"), "Zm8=");
}

#[test]
/// Positive test: Encodes 3 bytes without padding
fn test_encode_three_bytes() {
    assert_eq!(encode(b"foo"), "Zm9v");
}

#[test]
/// Positive test: Encodes 4 bytes with padding
fn test_encode_four_bytes() {
    assert_eq!(encode(b"foob"), "Zm9vYg==");
}

#[test]
/// Positive test: Encodes longer string — "Hack the Planet!" from Hackers (1995)
fn test_encode_hack_the_planet() {
    assert_eq!(encode(HACK_THE_PLANET), HACK_THE_PLANET_ENCODED);
}

#[test]
/// Positive test: Encodes all byte values (0-254) and verifies roundtrip
fn test_encode_all_bytes() {
    let bytes = (0..255).collect::<Vec<u8>>();
    let encoded = encode(&bytes);
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, bytes);
}

#[test]
/// Positive test: Decodes valid Base64 string — "Hack the Planet!" from Hackers (1995)
fn test_decode_valid() {
    assert_eq!(decode(HACK_THE_PLANET_ENCODED).unwrap(), HACK_THE_PLANET);
}

#[test]
/// Positive test: Decodes empty string
fn test_decode_empty() {
    assert_eq!(decode("").unwrap(), Vec::<u8>::new());
}

#[test]
/// Positive test: Decodes 1 group with 2 padding chars
fn test_decode_one_group() {
    assert_eq!(decode("Zg==").unwrap(), b"f");
}

#[test]
/// Positive test: Decodes 2 groups with 1 padding char
fn test_decode_two_groups() {
    assert_eq!(decode("Zm8=").unwrap(), b"fo");
}

#[test]
/// Positive test: Decodes 3 groups without padding
fn test_decode_three_groups() {
    assert_eq!(decode("Zm9v").unwrap(), b"foo");
}

#[test]
/// Negative test: Rejects strings whose length is not a multiple of 4
fn test_decode_invalid_length() {
    assert!(decode("a").is_err());
    assert!(decode("ab").is_err());
    assert!(decode("abc").is_err());
    assert!(decode("abcde").is_err());
    assert!(decode("AAAA=").is_err()); // padding makes length invalid
}

#[test]
/// Negative test: Rejects invalid Base64 characters
fn test_decode_invalid_char() {
    assert!(decode("!@#$").is_err());
}

#[test]
/// Negative test: Rejects non-ASCII characters (value > 127)
fn test_decode_non_ascii() {
    // é is multi byte UTF-8, both bytes > 127
    assert!(decode("éAAA").is_err());
}

#[test]
/// Positive test: Ensures encode/decode is reversible for a complex string
fn test_encode_decode_roundtrip() {
    let original = b"This is a test string with various characters: !@#$%^&*()";
    let encoded = encode(original);
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, original);
}

#[test]
/// Positive test: Decodes Base64 string with 1 padding char
fn test_decode_with_one_padding() {
    assert_eq!(decode("TWE=").unwrap(), b"Ma");
}

#[test]
/// Positive test: Decodes Base64 string with 2 padding chars
fn test_decode_with_two_padding() {
    assert_eq!(decode("TQ==").unwrap(), b"M");
}

#[test]
/// Positive test: Decodes Base64 string without padding
fn test_decode_without_padding() {
    assert_eq!(decode("TWFu").unwrap(), b"Man");
}
