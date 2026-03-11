// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

use std::fmt;
use std::time::SystemTimeError;

/// All possible errors from warrior-util
/// Created by claude
#[derive(Debug)]
pub enum UtilError {
    /// RwLock or Mutex was poisoned by a panicking thread
    // LockPoisioned does not contain that much useful information so we just use a string here
    LockPoisoned(String),
    /// IO error — file read or write failed
    IoError(std::io::Error),
    /// System clock error — time went backwards
    TimeError(SystemTimeError),
    /// Invalid input provided by caller
    InvalidInput(String),
}

/// Generates a [`From`] impl to convert a foreign error type into [`UtilError`].
///
/// This reduces boilerplate of implementing the ['From'] trait since they are all the same
///
/// # Examples
/// ```ignore
/// impl_from_error!(SystemTimeError, TimeError);
/// impl_from_error!(std::io::Error, IoError);
/// ```
macro_rules! impl_from_error {
    ($from_type: ty, $error: ident) => {
        impl From<$from_type> for UtilError {
            fn from(value: $from_type) -> Self {
                Self::$error(value)
            }
        }
    };
}

// Implement the From trait for basic wrapping error
impl_from_error!(SystemTimeError, TimeError);
impl_from_error!(std::io::Error, IoError);

/// Display — human readable messages for each error
impl fmt::Display for UtilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UtilError::LockPoisoned(msg) => write!(f, "RwLock poisoned: {}", msg),

            UtilError::IoError(e) => write!(f, "IO error: {}", e),

            UtilError::TimeError(e) => write!(f, "system clock error: {}", e),

            UtilError::InvalidInput(msg) => write!(f, "invalid input: {}", msg),
        }
    }
}

/// Error trait — body is empty because Debug and Display
impl std::error::Error for UtilError {
    /// Expose underlying error source for wrapped errors
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            UtilError::IoError(e) => Some(e),
            UtilError::TimeError(e) => Some(e),
            _ => None,
        }
    }
}
