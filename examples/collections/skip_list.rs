// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

use warrior_util::collections::skip_list::SkipList;

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
        println!("  Inserted: {}", value);
    }

    println!("\n2. Visual representation of the SkipList:");
    println!("{}", list);

    println!("\n3. Searching for elements:");
    for &value in &[1, 5, 10, 15, 20, 25] {
        let found = list.search(value);
        println!("  Search for {}: {}", value, if found { "Found" } else { "Not found" });
    }

    println!("\n4. SkipList properties:");
    println!("  - Maintains sorted order automatically");
    println!("  - Average O(log n) time for insert/search");
    println!("  - Uses probabilistic balancing (coin flips)");
    println!("  - No duplicates allowed (silently ignored)");

    println!("\n5. Demonstrating duplicate insertion:");
    println!("  Attempting to insert 10 again...");
    list.insert(10); // This will be ignored
    println!("  Search for 10: {}", if list.search(10) { "Still found" } else { "Lost!" });

    println!("\n6. Large dataset performance:");
    let mut large_list = SkipList::new();
    for i in (0..100).rev() { // Insert in reverse order
        large_list.insert(i);
    }
    println!("  Inserted 100 elements in reverse order");
    println!("  Search for 50: {}", if large_list.search(50) { "Found" } else { "Not found" });
    println!("  Search for 150: {}", if large_list.search(150) { "Found" } else { "Not found" });
}
