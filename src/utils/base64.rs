/// Base 64 charset as bytes(only need u8 for these specific chars)
const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
/// Bit mask to get lower 6 bits from 8 bit number
const SIX_BIT_MASK_U8: u8 = 0x3F;
/// Bit mask to get lower 6 bits from 16 bit number
const SIX_BIT_MASK_U16: u16 = 0x3F;
/// Bit mask to get lower 6 bits from 32 bit number
const SIX_BIT_MASK: u32 = 0x3F;
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
/// let base_64_str: String = base64::encode(b"Hello, World!", "SGVsbG8sIFdvcmxkIQ==");
/// ```
pub fn encode(bytes: &[u8]) -> String {
    //we need first 3 bytes
    //lets add padding bytes
    let remainder = bytes.len() % 3;
    let mut base64_string = String::new();
    for chunk in bytes[..bytes.len() - remainder].chunks(3) {
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
            let byte_1: u8 = bytes[bytes.len() - 2];
            let byte_2: u8 = bytes[bytes.len() - 1];
            //combine the bytes
            let combined: u16 = (byte_1 as u16) << 8 | byte_2 as u16;
            // 16 - 6 = 10 <- get first 6 bits
            let char_1: char = CHARS[((combined >> 10) & SIX_BIT_MASK_U16) as usize] as char;
            //next 6 bits
            let char_2: char = CHARS[((combined >> 4) & SIX_BIT_MASK_U16) as usize] as char;
            // we need 16 - 12 = 4 bits left, shift right over 2 so 6 bit mask works as expected
            let char_3: char = CHARS[((combined << 2) & SIX_BIT_MASK_U16) as usize] as char;
            base64_string.push(char_1);
            base64_string.push(char_2);
            base64_string.push(char_3);
            //2 remaining bytes add 1 padding char
            base64_string.push_str("=");
        }
        _ => unreachable!(), // x mod 3 will always be between 0-2(galois died for this)
    }

    base64_string
}

pub fn decode(base_64_str: &str) -> Vec<u8> {
    unimplemented!();
}
