// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

use crate::utils::errors::UtilError;
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current time as milliseconds since UNIX epoch.
///
/// # Errors
/// * `UtilError::SystemTimeError` if the system clock is before UNIX epoch
pub fn current_time_millis() -> Result<u64, UtilError> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64)
}
