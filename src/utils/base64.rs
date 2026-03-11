// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

use crate::utils::errors::UtilError;

/// Base 64 charset as bytes(only need u8 for these specific chars)
const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
/// Bit mask to get lower 6 bits from 8 bit number
const SIX_BIT_MASK_U8: u8 = 0x3F;
/// Bit mask to get lower 6 bits from 16 bit number
const SIX_BIT_MASK_U16: u16 = 0x3F;
/// Bit mask to get lower 6 bits from 32 bit number
const SIX_BIT_MASK: u32 = 0x3F;

///Decode table for fast lookups of 6 bit value based on decimal value of base64 char
const DECODE_TABLE: [u8; 128] = {
    let mut table = [255u8; 128]; // 255 = invalid marker

    //add letters since they are next to eachother in ASCII table we can add i for each letter
    let mut i = 0;
    while i < 26 {
        table[(b'A' + i) as usize] = i; // A-Z = 0-25
        table[(b'a' + i) as usize] = i + 26; // a-z = 26-51
        i += 1;
    }

    //same is true with numbers
    let mut i = 0;
    while i < 10 {
        table[(b'0' + i) as usize] = i + 52; // 0-9 = 52-61
        i += 1;
    }

    //add last few values
    table[b'+' as usize] = 62;
    table[b'/' as usize] = 63;
    table[b'=' as usize] = 0;
    table
};

/// Encodes a byte slice into a Base64 encoded string.
///
/// # Arguments
/// * `bytes` - The byte slice to encode
///
/// # Returns
/// A Base64 encoded `String` padded with '=' as necessary
///
/// # Example
/// ```
/// use warrior_util::utils::base64;
///
/// let base_64_str: String = base64::encode(b"Hello, World!"); // SGVsbG8sIFdvcmxkIQ==
/// ```
pub fn encode(bytes: &[u8]) -> String {
    let remainder = bytes.len() % 3;
    let mut base64_string = String::new();

    //iterate thru each 3 byte chunk(divisible by 6) and deal with remainder later
    for chunk in bytes[0..bytes.len() - remainder].chunks(3) {
        debug_assert!(chunk.len() == 3, "Chunks should always have size of 3!");

        //combine the 3 chunks(8 bit) together to get 24 bit binary number, Big endian MSB first
        let combined: u32 = (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8 | (chunk[2] as u32);

        //convert number into numbers we need to shift over 18 to get first 6 bits (24 - 18 = 6)
        let char_1: char = CHARS[(combined >> 18 & SIX_BIT_MASK) as usize] as char;
        let char_2: char = CHARS[(combined >> 12 & SIX_BIT_MASK) as usize] as char;
        let char_3: char = CHARS[(combined >> 6 & SIX_BIT_MASK) as usize] as char;
        let char_4: char = CHARS[(combined & SIX_BIT_MASK) as usize] as char;

        base64_string.push(char_1);
        base64_string.push(char_2);
        base64_string.push(char_3);
        base64_string.push(char_4);
    }

    //handle the padding
    match remainder {
        0 => { /* do nothing - no extra bytes */ }
        1 => {
            //get remaining byte
            let last_byte: u8 = bytes[bytes.len() - 1];
            // 8 - 6 = 2
            let char_1: char = CHARS[((last_byte >> 2) & SIX_BIT_MASK_U8) as usize] as char;
            // move over 4 since we use the same first 6 bit mask
            let char_2: char = CHARS[((last_byte << 4) & SIX_BIT_MASK_U8) as usize] as char;
            base64_string.push(char_1);
            base64_string.push(char_2);
            //only 1 byte so add 2 padding char
            base64_string.push('=');
            base64_string.push('=');
        }
        2 => {
            //get remaining bytes
            let byte_1: u16 = bytes[bytes.len() - 2] as u16;
            let byte_2: u16 = bytes[bytes.len() - 1] as u16;
            //combine the bytes
            let combined: u16 = byte_1 << 8 | byte_2;
            // 16 - 6 = 10 <- get first 6 bits
            let char_1: char = CHARS[((combined >> 10) & SIX_BIT_MASK_U16) as usize] as char;
            //next 6 bits
            let char_2: char = CHARS[((combined >> 4) & SIX_BIT_MASK_U16) as usize] as char;
            // we need 16 - 12 = 4 bits left, shift right over 2 so 6 bit mask works as expected
            let char_3: char = CHARS[((combined << 2) & SIX_BIT_MASK_U16) as usize] as char;
            //push chars
            base64_string.push(char_1);
            base64_string.push(char_2);
            base64_string.push(char_3);
            // Add 1 padding char
            base64_string.push('=');
        }
        _ => unreachable!(), // x mod 3 will always be between 0-2(galois died for this)
    }

    base64_string
}

/// Decodes a Base64 encoded string into a byte vector.
///
/// # Arguments
/// * `base_64_str` - A Base64 encoded string to decode
///
/// # Returns
/// A `Result` containing the decoded bytes as `Vec<u8>`, or a `String` error
/// if the input is invalid.
///
/// # Errors
/// * If the input length is not a multiple of 4
/// * If the input contains invalid Base64 characters
///
/// # Examples
/// ```
/// use warrior_util::utils::base64::{decode};
///
/// assert_eq!(decode("TWFu").unwrap(), b"Man");
/// assert!(decode("invalid!").is_err());
/// ```
pub fn decode(base_64_str: &str) -> Result<Vec<u8>, UtilError> {
    if base_64_str.len() % 4 != 0 {
        return Err(UtilError::InvalidInput(format!(
            "Invalid Base 64 string length: {}",
            base_64_str.len()
        )));
    }

    let mut bytes: Vec<u8> = Vec::new();

    for chunks in base_64_str.as_bytes().chunks(4) {
        let char_1: u32 = lookup_base64_char(chunks[0])? as u32;
        let char_2: u32 = lookup_base64_char(chunks[1])? as u32;
        let char_3: u32 = lookup_base64_char(chunks[2])? as u32;
        let char_4: u32 = lookup_base64_char(chunks[3])? as u32;
        //each base 64 char represents some 6 bit number so we shift each by 6
        let combined: u32 = char_1 << 18 | char_2 << 12 | char_3 << 6 | char_4;

        //we get the three bytes from base64 char (6 * 4 / 8 = 3)
        bytes.push((combined >> 16) as u8);
        bytes.push((combined >> 8) as u8);
        bytes.push(combined as u8);
    }

    //remove the two bytes of padding
    if base_64_str.ends_with("==") {
        bytes.truncate(bytes.len() - 2);
    }
    //remove the one byte of padding
    else if base_64_str.ends_with("=") {
        bytes.truncate(bytes.len() - 1);
    }

    Ok(bytes)
}

/// Looks up the 6 bit Base64 index for a given ASCII character using the decode table.
/// # Errors
/// * If the character is not valid ASCII (value > 127)
/// * If the character is not in the Base64 alphabet
fn lookup_base64_char(c: u8) -> Result<u8, UtilError> {
    // base64 only uses ASCII characters so anything above 127 is invalid
    if c > 127 {
        return Err(UtilError::InvalidInput(format!(
            "Invalid Base64 character {} ",
            c as char
        )));
    }

    // look up the 6 bit index in the decode table
    // table is pre-populated with 255 for any non base64 character
    let index = DECODE_TABLE[c as usize];

    // character is not in the base64 alphabet
    if index == 255 {
        return Err(UtilError::InvalidInput(format!(
            "Invalid Base64 character {} ",
            c as char
        )));
    }

    Ok(index)
}
