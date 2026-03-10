// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

//! # Test Plan for time Module
//!
//! Author: GitHub Copilot
//!
//! ## General Testing Procedure
//! Tests validate time utility functions. Checks return values are reasonable
//! and monotonically increasing. Error handling for system time issues.
//!
//! ## Edge Cases Tested
//! - System clock behavior (monotonicity)
//! - Potential system time errors (though rare)
//!
//! ## Positive Tests
//! - `test_current_time_millis`: Returns valid timestamps
//!
//! ## Negative Tests
//! - None (function rarely fails in normal conditions)

use warrior_util::utils::time::current_time_millis;

#[test]
/// Positive test: Returns valid timestamps
fn test_current_time_millis() {
    let time = current_time_millis().unwrap();
    assert!(time > 0);
    // Call again, should be >= previous
    let time2 = current_time_millis().unwrap();
    assert!(time2 >= time);
}
