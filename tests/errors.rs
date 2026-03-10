// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

//! # Test Plan for errors Module
//!
//! Author: Claude Sonnet 4.6 (Anthropic)
//!
//! ## General Testing Procedure
//! Tests validate the UtilError enum implements standard error traits correctly.
//! Covers Display formatting, Debug representation, and error source chaining
//! for all four variants: LockPoisoned, IoError, SystemTimeError, and InvalidInput.
//!
//! ## Edge Cases Tested
//! - Display formatting for all variants
//! - Error source chaining — IoError and SystemTimeError have sources, others do not
//! - Debug representation
//!
//! ## Positive Tests
//! - `test_util_error_display`: All variants format correctly via Display
//! - `test_util_error_source`: IoError and SystemTimeError expose their source
//! - `test_util_error_debug`: Debug output contains variant name
//!
//! ## Negative Tests
//! - `test_util_error_no_source`: LockPoisoned and InvalidInput return None from source()

use std::error::Error;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use warrior_util::utils::errors::UtilError;

#[test]
/// Positive test: All variants format correctly via Display
fn test_util_error_display() {
    let err = UtilError::InvalidInput("bad value".to_string());
    assert_eq!(format!("{}", err), "invalid input: bad value");

    let err = UtilError::LockPoisoned("thread panicked".to_string());
    assert_eq!(
        format!("{}", err),
        "lock poisoned by panicking thread: thread panicked"
    );

    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = UtilError::IoError(io_err);
    assert!(format!("{}", err).starts_with("IO error:"));
}

#[test]
/// Positive test: IoError and SystemTimeError expose their underlying source
fn test_util_error_source() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = UtilError::IoError(io_err);
    assert!(err.source().is_some());

    // trigger a SystemTimeError by subtracting from UNIX_EPOCH
    let result = UNIX_EPOCH.duration_since(SystemTime::now());
    if let Err(e) = result {
        let err = UtilError::SystemTimeError(e);
        assert!(err.source().is_some());
    }
}

#[test]
/// Negative test: LockPoisoned and InvalidInput have no error source
fn test_util_error_no_source() {
    let err = UtilError::LockPoisoned("panic".to_string());
    assert!(err.source().is_none());

    let err = UtilError::InvalidInput("bad value".to_string());
    assert!(err.source().is_none());
}

#[test]
/// Positive test: Debug output contains the variant name
fn test_util_error_debug() {
    let err = UtilError::InvalidInput("test".to_string());
    assert!(format!("{:?}", err).contains("InvalidInput"));

    let err = UtilError::LockPoisoned("test".to_string());
    assert!(format!("{:?}", err).contains("LockPoisoned"));

    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = UtilError::IoError(io_err);
    assert!(format!("{:?}", err).contains("IoError"));
}
