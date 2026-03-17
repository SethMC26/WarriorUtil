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
    println!("Inserted values: {:?}", values);

    println!("\n2. Visual representation of the SkipList:");
    println!("{}", list);

    println!("\n3. Checking if elements exist:");
    for &value in &[5, 10, 25] {
        let exists = list.exists(value);
        println!("Does {} exist? {}", value, exists);
    }

    println!("\n4. Getting elements:");
    for &value in &[5, 10, 25] {
        match list.get(&value) {
            Some(val) => println!("Got {}: {}", value, val),
            None => println!("{} not found", value),
        }
    }

    println!("\n5. Deleting elements:");
    println!("Before deletion:");
    println!("{}", list);
    list.delete(&5);
    list.delete(&15);
    list.delete(&25); // doesn't exist, should do nothing
    println!("After deleting 5 and 15:");
    println!("{}", list);

    println!("\n6. Large dataset example:");
    let mut large_list = SkipList::new();
    for i in 0..25 { // Insert in reverse order
        large_list.insert(i);
    }   
    println!("Large list with 25 elements:");
    println!("{}", large_list);

    println!("\n7. Using the skip_list! macro:");
    let macro_list = skip_list![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("List created with macro:");
    println!("{}", macro_list);
}
