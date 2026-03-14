// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

//! # Test Plan for SkipList Collection
//!
//! Author: Claude Sonnet 4.6 (Anthropic)
//!
//! ## General Testing Procedure
//! Integration tests validate SkipList insertion and search operations.
//! Tests cover various insertion orders, search scenarios, and edge cases.
//! SkipList uses probabilistic balancing with coin flips for level promotion.
//!
//! ## Edge Cases Tested
//! - Empty list operations
//! - Single element lists
//! - Insertion at beginning, middle, and end
//! - Duplicate insertions (silently ignored)
//! - Large lists (100+ elements)
//! - Boundary values (min/max i32)
//! - Negative numbers
//! - Sequential and random insertion orders
//!
//! ## Positive Tests
//! - `test_insert_*`: Various insertion scenarios maintain sorted order
//! - `test_search_*`: Search finds existing elements, misses non-existing
//! - `test_search_duplicate`: Duplicate inserts don't affect search
//!
//! ## Negative Tests
//! - `test_search_empty`: Search on empty list returns false
//! - `test_search_missing`: Search for non-existent values returns false

use warrior_util::collections::skip_list::SkipList;

#[test]
/// Positive test: Inserts into empty list
fn test_insert_empty() {
    let mut list = SkipList::new();
    list.insert(5);
    assert!(list.search(5));
}

#[test]
/// Positive test: Inserts middle then end values
fn test_insert_middle_then_end() {
    let mut list = SkipList::new();
    list.insert(10);
    list.insert(20);
    list.insert(15); // middle
    list.insert(25); // end
    assert!(list.search(10));
    assert!(list.search(15));
    assert!(list.search(20));
    assert!(list.search(25));
}

#[test]
/// Positive test: Inserts middle then beginning values
fn test_insert_middle_then_beginning() {
    let mut list = SkipList::new();
    list.insert(10);
    list.insert(20);
    list.insert(15); // middle
    list.insert(5); // beginning
    assert!(list.search(5));
    assert!(list.search(10));
    assert!(list.search(15));
    assert!(list.search(20));
}

#[test]
/// Positive test: Inserts middle then both ends
fn test_insert_middle_then_both_ends() {
    let mut list = SkipList::new();
    list.insert(10);
    list.insert(20);
    list.insert(15); // middle
    list.insert(25); // end
    list.insert(5); // beginning
    assert!(list.search(5));
    assert!(list.search(10));
    assert!(list.search(15));
    assert!(list.search(20));
    assert!(list.search(25));
}

#[test]
/// Positive test: Inserts multiple middle values
fn test_insert_multiple_middle() {
    let mut list = SkipList::new();
    list.insert(10);
    list.insert(40);
    list.insert(20); // middle
    list.insert(30); // middle
    list.insert(25); // middle between middles
    list.insert(50); // end
    list.insert(5); // beginning
    for &val in &[5, 10, 20, 25, 30, 40, 50] {
        assert!(list.search(val), "Should find {}", val);
    }
}

#[test]
/// Positive test: Inserts adjacent middle values
fn test_insert_adjacent_middle() {
    let mut list = SkipList::new();
    list.insert(10);
    list.insert(30);
    list.insert(20); // middle
    list.insert(21); // adjacent to middle
    list.insert(19); // just before middle
    for &val in &[10, 19, 20, 21, 30] {
        assert!(list.search(val), "Should find {}", val);
    }
}

#[test]
/// Negative test: Search on empty list returns false
fn test_search_empty() {
    let list: SkipList<i32> = SkipList::new();
    assert!(!list.search(1));
}

#[test]
/// Positive test: Search finds single element
fn test_search_single_element() {
    let mut list = SkipList::new();
    list.insert(5);
    assert!(list.search(5));
    assert!(!list.search(3));
}

#[test]
/// Positive test: Search finds head element
fn test_search_head() {
    let mut list = SkipList::new();
    list.insert(5);
    list.insert(3);
    assert!(list.search(3));
}

#[test]
/// Positive test: Search finds tail element
fn test_search_tail() {
    let mut list = SkipList::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    assert!(list.search(3));
}

#[test]
/// Negative test: Search misses non-existent values
fn test_search_missing() {
    let mut list = SkipList::new();
    list.insert(1);
    list.insert(3);
    list.insert(5);
    assert!(!list.search(2));
    assert!(!list.search(4));
    assert!(!list.search(6));
}

#[test]
/// Positive test: Search works on large lists
fn test_search_many() {
    let mut list = SkipList::new();
    for i in 0..100 {
        list.insert(i);
    }
    for i in 0..100 {
        assert!(list.search(i), "should find {}", i);
    }
    assert!(!list.search(100));
    assert!(!list.search(-1));
}

#[test]
/// Positive test: Search finds new minimum after insertions
fn test_search_new_minimum() {
    let mut list = SkipList::new();
    list.insert(5);
    list.insert(3);
    list.insert(1);
    assert!(list.search(1));
    assert!(list.search(3));
    assert!(list.search(5));
}

#[test]
/// Positive test: Duplicate insertions don't affect search
fn test_search_duplicate() {
    let mut list = SkipList::new();
    list.insert(5);
    list.insert(5);
    assert!(list.search(5));
}

#[test]
/// Positive test: Handles negative numbers
fn test_negative_numbers() {
    let mut list = SkipList::new();
    list.insert(-10);
    list.insert(0);
    list.insert(10);
    assert!(list.search(-10));
    assert!(list.search(0));
    assert!(list.search(10));
}

#[test]
/// Positive test: Handles boundary i32 values
fn test_boundary_values() {
    let mut list = SkipList::new();
    list.insert(i32::MIN);
    list.insert(0);
    list.insert(i32::MAX);
    assert!(list.search(i32::MIN));
    assert!(list.search(0));
    assert!(list.search(i32::MAX));
}

#[test]
/// Positive test: Random insertion order maintains correctness
fn test_random_insertion_order() {
    let mut list = SkipList::new();
    let values = vec![42, 17, 99, 3, 67, 23, 88, 1, 55, 76];
    for &val in &values {
        list.insert(val);
    }
    for &val in &values {
        assert!(list.search(val), "Should find {}", val);
    }
    assert!(!list.search(100)); // not inserted
}
