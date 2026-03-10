//! # Test Plan for cli Module
//!
//! Author: Claude Sonnet 4.6 (Anthropic)
//!
//! ## General Testing Procedure
//! Tests validate command-line argument parsing structures and helper functions.
//! Since `get_op_map` depends on `std::env::args`, tests focus on data structures
//! and helper functions. Hash and equality tests use HashSet insertion behavior.
//!
//! ## Edge Cases Tested
//! - Option creation with and without arguments
//! - Empty option lists for string formatting
//! - Hash and equality behavior for LongOp
//!
//! ## Positive Tests
//! - `test_long_op_new`: Creates options with correct fields
//! - `test_long_op_has_arg`: Modifies argument requirement
//! - `test_options_string_with_ops`: Formats option help text
//! - `test_long_op_hash_eq`: Validates Hash and Eq implementations
//!
//! ## Negative Tests
//! - `test_options_string_empty`: Handles empty option lists

use std::collections::HashSet;
use warrior_util::utils::cli::{options_string, LongOp};

#[test]
/// Positive test: Creates a LongOp with correct fields, has_arg defaults to true
fn test_long_op_new() {
    let op = LongOp::new("p", "port", "Port to use");
    assert_eq!(op.short_op, "p");
    assert_eq!(op.long_op, "port");
    assert_eq!(op.usage, "Port to use");
    assert!(op.has_arg);
}

#[test]
/// Positive test: has_arg can be set to false via builder pattern
fn test_long_op_has_arg() {
    let op = LongOp::new("h", "help", "Display help").has_arg(false);
    assert!(!op.has_arg);
}

#[test]
/// Negative test: Empty option list produces header with no options
fn test_options_string_empty() {
    let ops: Vec<LongOp> = vec![];
    let s = options_string(&ops);
    assert_eq!(s, "Options: \n");
}

#[test]
/// Positive test: Options string contains short op, long op and usage for each option
fn test_options_string_with_ops() {
    let ops = vec![
        LongOp::new("p", "port", "Port to use"),
        LongOp::new("h", "help", "Display help").has_arg(false),
    ];
    let s = options_string(&ops);
    assert!(s.contains("-p, --port"));
    assert!(s.contains("-h, --help"));
    assert!(s.contains("Port to use"));
    assert!(s.contains("Display help"));
}

#[test]
/// Positive test: Two LongOps with same fields are equal and produce the same hash
fn test_long_op_hash_eq() {
    let op1 = LongOp::new("p", "port", "Port");
    let op2 = LongOp::new("p", "port", "Port");
    let op3 = LongOp::new("h", "help", "Help");

    let mut set = HashSet::new();
    set.insert(op1);
    assert!(!set.insert(op2));  // duplicate — should not insert
    set.insert(op3);
    assert_eq!(set.len(), 2);
}