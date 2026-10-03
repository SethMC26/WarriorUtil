// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

use std::fs::File;
use std::io::Read;

use crate::utils::errors::UtilError;
static RANDOM_SOUCE: &str = "/dev/urandom";

/// Get random bytes from /dev/urandom this is a hacky quick fix we will need to find a good crypto crate later
/// TODO update with crypto crate
/// Claude generated function
pub fn fill_random_bytes(buf: &mut [u8]) -> Result<(), UtilError> {
    let mut f:File = File::open(RANDOM_SOUCE)?;
    f.read_exact(buf)?;
    Ok(())
}

/// Generates Random byte buffer using sytems randomness source. 
pub fn get_random_bytes_buffer(size: usize) -> Result<Vec<u8>, UtilError> {
    let mut file: File = File::open(RANDOM_SOUCE)?;
    let mut buf: Vec<u8> = vec![0u8; size]; 
    file.read_exact(&mut buf)?;

    Ok(buf)
}

/// Generated true/false based on system randomness source. 
pub fn coin_flip() -> Result<bool, UtilError> {
    let mut file: File = File::open(RANDOM_SOUCE)?;
    let mut buf: [u8; 1] = [0u8];
    file.read_exact(&mut buf)?;

    Ok(buf[0] % 2 == 0)
}