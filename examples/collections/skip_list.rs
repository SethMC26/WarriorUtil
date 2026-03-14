// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

use warrior_util::collections::skip_list::SkipList;

fn main() {
    let mut list: SkipList<i32> = SkipList::new();

    println!("{}", list);
    //list.insert(5);
    list.insert(1);
    list.insert(3);
    println!("{}", list);
    list.insert(6);
    println!("{}", list);
    list.insert(-2);
    println!("{}", list);
    list.insert(10);
    list.insert(20);
    list.insert(15); // middle
    list.insert(25); // end
    println!("{}", list);
    list.insert(-5); // beginning
    list.insert(5);
    list.insert(40);
    list.insert(45);
    list.insert(50);
    list.insert(55);
    list.insert(43);
    list.insert(53);
    list.insert(60);
    println!("{}", list)
}
