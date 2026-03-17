// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

use warrior_util::{collections::skip_list::SkipList, skip_list};

fn main() {
    //VIBED CODED ALERT THE EXAMPLES HAS BEEN VIBED
    println!("SkipList Example - Probabilistic Data Structure");
    println!("==============================================\n");

    // Create a new empty SkipList
    // SkipList is a probabilistic data structure that provides O(log n) average time
    // complexity for insert and search operations
    let mut list = SkipList::new();

    println!("1. Inserting elements in random order:");
    let values = [10, 5, 15, 3, 7, 12, 18, 1, 20];
    for &value in &values {
        list.insert(value);
    }

    println!("\n2. Visual representation of the SkipList:");
    println!("{}", list);

    println!("\n6. Large dataset performance:");
    let mut large_list = SkipList::new();
    for i in (0..25).into_iter() {
        // Insert in reverse order
        large_list.insert(i);
    }

    println!("{}", large_list);

    //also use skip_list macro
    let list = skip_list![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{}", list);
}
