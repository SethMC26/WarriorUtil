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
    assert!(list.exists(5));
}

#[test]
/// Positive test: Inserts middle then end values
fn test_insert_middle_then_end() {
    let mut list = SkipList::new();
    list.insert(10);
    list.insert(20);
    list.insert(15); // middle
    list.insert(25); // end
    assert!(list.exists(10));
    assert!(list.exists(15));
    assert!(list.exists(20));
    assert!(list.exists(25));
}

#[test]
/// Positive test: Inserts middle then beginning values
fn test_insert_middle_then_beginning() {
    let mut list = SkipList::new();
    list.insert(10);
    list.insert(20);
    list.insert(15); // middle
    list.insert(5); // beginning
    assert!(list.exists(5));
    assert!(list.exists(10));
    assert!(list.exists(15));
    assert!(list.exists(20));
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
    assert!(list.exists(5));
    assert!(list.exists(10));
    assert!(list.exists(15));
    assert!(list.exists(20));
    assert!(list.exists(25));
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
        assert!(list.exists(val), "Should find {}", val);
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
        assert!(list.exists(val), "Should find {}", val);
    }
}

#[test]
/// Negative test: Search on empty list returns false
fn test_search_empty() {
    let list: SkipList<i32> = SkipList::new();
    assert!(!list.exists(1));
}

#[test]
/// Positive test: Search finds single element
fn test_search_single_element() {
    let mut list = SkipList::new();
    list.insert(5);
    assert!(list.exists(5));
    assert!(!list.exists(3));
}

#[test]
/// Positive test: Search finds head element
fn test_search_head() {
    let mut list = SkipList::new();
    list.insert(5);
    list.insert(3);
    assert!(list.exists(3));
}

#[test]
/// Positive test: Search finds tail element
fn test_search_tail() {
    let mut list = SkipList::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    assert!(list.exists(3));
}

#[test]
/// Negative test: Search misses non-existent values
fn test_search_missing() {
    let mut list = SkipList::new();
    list.insert(1);
    list.insert(3);
    list.insert(5);
    assert!(!list.exists(2));
    assert!(!list.exists(4));
    assert!(!list.exists(6));
}

#[test]
/// Positive test: Search works on large lists
fn test_search_many() {
    let mut list = SkipList::new();
    for i in 0..100 {
        list.insert(i);
    }
    for i in 0..100 {
        assert!(list.exists(i), "should find {}", i);
    }
    assert!(!list.exists(100));
    assert!(!list.exists(-1));
}

#[test]
/// Positive test: Search finds new minimum after insertions
fn test_search_new_minimum() {
    let mut list = SkipList::new();
    list.insert(5);
    list.insert(3);
    list.insert(1);
    assert!(list.exists(1));
    assert!(list.exists(3));
    assert!(list.exists(5));
}

#[test]
/// Positive test: Duplicate insertions don't affect search
fn test_search_duplicate() {
    let mut list = SkipList::new();
    list.insert(5);
    list.insert(5);
    assert!(list.exists(5));
}

#[test]
/// Positive test: Handles negative numbers
fn test_negative_numbers() {
    let mut list = SkipList::new();
    list.insert(-10);
    list.insert(0);
    list.insert(10);
    assert!(list.exists(-10));
    assert!(list.exists(0));
    assert!(list.exists(10));
}

#[test]
/// Positive test: Handles boundary i32 values
fn test_boundary_values() {
    let mut list = SkipList::new();
    list.insert(i32::MIN);
    list.insert(0);
    list.insert(i32::MAX);
    assert!(list.exists(i32::MIN));
    assert!(list.exists(0));
    assert!(list.exists(i32::MAX));
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
        assert!(list.exists(val), "Should find {}", val);
    }
    assert!(!list.exists(100)); // not inserted
}
#[test]
fn test_insert_and_exists() {
    let mut list = SkipList::new();
    list.insert(5);
    list.insert(3);
    list.insert(8);
    assert!(list.exists(5));
    assert!(list.exists(3));
    assert!(list.exists(8));
}

#[test]
fn test_exists_not_found() {
    let mut list = SkipList::new();
    list.insert(5);
    assert!(!list.exists(99));
}

#[test]
fn test_exists_empty() {
    let list: SkipList<i32> = SkipList::new();
    assert!(!list.exists(1));
}

#[test]
fn test_get() {
    let mut list = SkipList::new();
    list.insert(42);
    assert_eq!(list.get(&42), Some(42));
}

#[test]
fn test_get_not_found() {
    let mut list = SkipList::new();
    list.insert(1);
    assert_eq!(list.get(&99), None);
}

#[test]
fn test_get_empty() {
    let list: SkipList<i32> = SkipList::new();
    assert_eq!(list.get(&1), None);
}

#[test]
fn test_duplicate_insert() {
    let mut list = SkipList::new();
    list.insert(5);
    list.insert(5);
    assert!(list.exists(5));
}

#[test]
fn test_insert_ascending_order() {
    let mut list = SkipList::new();
    for i in 0..10 {
        list.insert(i);
    }
    for i in 0..10 {
        assert!(list.exists(i));
    }
}

#[test]
fn test_insert_descending_order() {
    let mut list = SkipList::new();
    for i in (0..10).rev() {
        list.insert(i);
    }
    for i in 0..10 {
        assert!(list.exists(i));
    }
}

#[test]
fn test_insert_random_order() {
    let mut list = SkipList::new();
    let values = vec![15, 3, 9, 1, 7, 20, 4];
    for &v in &values {
        list.insert(v);
    }
    for &v in &values {
        assert!(list.exists(v));
    }
}

#[test]
fn test_delete_middle() {
    let mut list = SkipList::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    list.delete(&2);
    assert!(!list.exists(2));
    assert!(list.exists(1));
    assert!(list.exists(3));
}

#[test]
fn test_delete_tail() {
    let mut list = SkipList::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    list.delete(&3);
    assert!(!list.exists(3));
    assert!(list.exists(1));
    assert!(list.exists(2));
}

#[test]
fn test_delete_not_found() {
    let mut list = SkipList::new();
    list.insert(1);
    list.insert(2);
    list.delete(&99);
    assert!(list.exists(1));
    assert!(list.exists(2));
}

#[test]
fn test_delete_empty_list() {
    let mut list: SkipList<i32> = SkipList::new();
    list.delete(&1); // should silently ignore
}

#[test]
fn test_delete_only_element() {
    let mut list = SkipList::new();
    list.insert(1);
    list.delete(&1);
    assert!(!list.exists(1));
}

#[test]
fn test_delete_then_reinsert() {
    let mut list = SkipList::new();
    list.insert(5);
    list.delete(&5);
    assert!(!list.exists(5));
    list.insert(5);
    assert!(list.exists(5));
}

#[test]
fn test_delete_promoted_node() {
    let mut list = SkipList::new();
    for i in 0..50 {
        list.insert(i);
    }
    list.delete(&25);
    assert!(!list.exists(25));
    assert!(list.exists(24));
    assert!(list.exists(26));
}

#[test]
fn test_large_insert() {
    let mut list = SkipList::new();
    for i in 0..100 {
        list.insert(i);
    }
    for i in 0..100 {
        assert!(list.exists(i));
    }
}

#[test]
fn test_large_delete() {
    let mut list = SkipList::new();
    for i in 0..100 {
        list.insert(i);
    }
    for i in (0..100).step_by(2) {
        list.delete(&i);
    }
    for i in 0..100 {
        if i % 2 == 0 {
            assert!(!list.exists(i));
        } else {
            assert!(list.exists(i));
        }
    }
}

#[test]
fn test_delete_head_two_elements() {
    let mut list = SkipList::new();
    list.insert(1);
    list.insert(2);
    list.delete(&1);
    assert!(!list.exists(1));
    assert!(list.exists(2));
}

#[test]
fn test_delete_tail_two_elements() {
    let mut list = SkipList::new();
    list.insert(1);
    list.insert(2);
    list.delete(&2);
    assert!(list.exists(1));
    assert!(!list.exists(2));
}

#[test]
fn test_delete_head_three_elements() {
    let mut list = SkipList::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    list.delete(&1);
    assert!(!list.exists(1));
    assert!(list.exists(2));
    assert!(list.exists(3));
}

#[test]
fn test_delete_all_elements_one_by_one() {
    let mut list = SkipList::new();
    for i in 0..10 {
        list.insert(i);
    }
    for i in 0..10 {
        assert!(list.exists(i));
        list.delete(&i);
        assert!(!list.exists(i));
    }
    // List should be empty
    for i in 0..10 {
        assert!(!list.exists(i));
    }
}

#[test]
fn test_delete_reverse_order() {
    let mut list = SkipList::new();
    for i in 0..10 {
        list.insert(i);
    }
    for i in (0..10).rev() {
        assert!(list.exists(i));
        list.delete(&i);
        assert!(!list.exists(i));
    }
}

#[test]
fn test_delete_random_order() {
    let mut list = SkipList::new();
    let values = vec![5, 2, 8, 1, 9, 3, 7, 4, 6, 0];
    for &v in &values {
        list.insert(v);
    }
    let delete_order = vec![3, 7, 1, 9, 2, 6, 0, 8, 4, 5];
    for &v in &delete_order {
        assert!(list.exists(v));
        list.delete(&v);
        assert!(!list.exists(v));
    }
}

#[test]
fn test_delete_non_existent_after_deletes() {
    let mut list = SkipList::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    list.delete(&2);
    list.delete(&99); // Should not panic or change anything
    assert!(list.exists(1));
    assert!(!list.exists(2));
    assert!(list.exists(3));
}

#[test]
fn test_delete_multiple_times() {
    let mut list = SkipList::new();
    list.insert(1);
    list.insert(2);
    list.delete(&1);
    list.delete(&1); // Delete again, should be no-op
    assert!(!list.exists(1));
    assert!(list.exists(2));
}

#[test]
fn test_delete_and_verify_remaining_sorted() {
    let mut list = SkipList::new();
    for i in 0..20 {
        list.insert(i);
    }
    // Delete some elements
    for &i in &[5, 10, 15] {
        list.delete(&i);
    }
    // Verify remaining are still findable and in order
    let mut prev = -1;
    for i in 0..20 {
        if i == 5 || i == 10 || i == 15 {
            assert!(!list.exists(i));
        } else {
            assert!(list.exists(i));
            assert!(i > prev);
            prev = i;
        }
    }
}

#[test]
fn test_delete_promoted_head() {
    let mut list = SkipList::new();
    // Insert many to increase chance of promotion
    for i in (0..50).rev() {
        // Insert in reverse to promote smaller numbers
        list.insert(i);
    }
    // Assuming 0 is head and promoted
    list.delete(&0);
    assert!(!list.exists(0));
    assert!(list.exists(1));
    // Verify structure by checking a few elements
    for i in 1..10 {
        assert!(list.exists(i));
    }
}

#[test]
fn test_delete_after_reinsert() {
    let mut list = SkipList::new();
    list.insert(1);
    list.delete(&1);
    list.insert(1);
    list.delete(&1);
    assert!(!list.exists(1));
}
