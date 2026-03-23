// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

use std::cell::RefCell;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use crate::debug_panic;

///type alias for node pointer
type Head<T> = Rc<RefCell<Node<T>>>;

/// A node in the skip list containing a value and forward pointers to the next node in each list that this node is in in.
///
/// `next_nodes[i]` holds the next node on list `i`.
struct Node<T: Ord> {
    value: T,
    next_nodes: Vec<Head<T>>,
}

impl<T> Node<T>
where
    T: Ord,
{
    /// Sets the forward pointer for a given list.
    /// pointer at `list` is overwritten.
    ///
    /// # Arguments
    /// * `next` - The node to point to at this level
    /// * `list` - The level index to set the forward pointer for
    pub fn set_next(&mut self, next: Head<T>, list: usize) {
        if self.next_nodes.len() <= list {
            self.next_nodes.push(next);
            return;
        }

        self.next_nodes[list] = next;
    }

    ///Removes the
    pub fn remove_next(&mut self, list: usize) {
        let Some(to_delete) = self.next_nodes.get(list).cloned() else {
            //use defensive assert(only triggers in debug) to help assert our worldview
            debug_panic!("No node to remove caller is using this function wrong");
            return;
        };

        let Some(new_next) = to_delete.borrow().next_nodes.get(list).cloned() else {
            self.next_nodes.pop();
            return;
        };

        self.next_nodes[list] = new_next;
    }
}

/// SkipList collection on average O(lg(n)) insert and search
/// We can insert any element that implements Ord(ordering) which is similar to comparable in java
/// `head` - Current Head of the skip list which points to all levels
#[derive(Default)]
pub struct SkipList<T: Ord> {
    head: Option<Head<T>>,
}

impl<T> SkipList<T>
where
    T: Ord,
{
    ///Creates a new empty skip list
    pub fn new() -> Self {
        SkipList { head: None }
    }

    ///Inserts `value` into this skip list
    ///
    /// # Notes
    /// Memory leaks? hope not but technically possible if seth is a shitty programmer and created a cycle with RC
    /// May require some code review and profiling
    /// # Panics
    /// If seth is a bad programmer and fucked up the RefCell Borrowing causing a runtime panic oops
    pub fn insert(&mut self, value: T) {
        //create new node
        let new_head: Head<T> = Rc::new(RefCell::new(Node {
            value: value,
            next_nodes: Vec::new(),
        }));

        //clones show up a lot in the code normally this might be bad practice but Head<T> Type is a RC, clones are cheap incrementing referrence count

        //root head is the head of all skip lists
        let root_head: Head<T> = match self.head.clone() {
            Some(head) => head,
            None => {
                //skip list is empty so just add new as head
                self.head = Some(new_head);
                return;
            }
        };

        //Check if new element should be new root head(smallest element)
        //for adding a new head we do not do coin flips since new head points to every skip list anyways
        if root_head.borrow().value > new_head.borrow().value {
            let mut new_node = new_head.borrow_mut();

            //have new node point to each list, skip first we already added it
            for node in &root_head.borrow().next_nodes {
                new_node.next_nodes.push(node.clone());
            }
            //add root head back to list
            new_node.set_next(root_head.clone(), 0);

            self.head = Some(new_head.clone());
            return;
        }

        //while we get "heads" promote node to the next
        let mut list_i: usize = 0;
        while self.coin_flip() {
            list_i += 1;
        }

        //add all new levels to skip lists, since level is new root_head just points at new node
        let old_len: usize = root_head.borrow().next_nodes.len();
        if list_i > old_len {
            let new_levels = list_i - old_len - 1;
            for _ in 0..new_levels {
                //use regular push since these are always new lists
                root_head.borrow_mut().next_nodes.push(new_head.clone());
            }
            list_i = old_len.saturating_sub(1);
        }

        //we need two lists one to hold  all prev for prev-> new_node and one for new_node->next
        //this is to avoid modifying skiplists while searching
        let mut prev_nodes: Vec<Head<T>> = Vec::new();
        let mut next_nodes: Vec<Head<T>> = Vec::new();

        //set curr_head is where we start search
        let mut curr_head: Head<T> = root_head;

        //search the skiplists in optimal way keeping track of where the new_node should get added
        loop {
            //get next node
            let Some(next_node) = curr_head.borrow().next_nodes.get(list_i).cloned() else {
                //since there is no next node the new_head is new tail
                prev_nodes.push(curr_head.clone());
                //go to next list or break
                list_i = match list_i.checked_sub(1) {
                    Some(i) => i,
                    None => break, //no list exists break loop
                };
                continue;
            };

            if next_node.borrow().value < new_head.borrow().value {
                curr_head = next_node;
            } else if next_node.borrow().value > new_head.borrow().value {
                //curr_head should go to new Node and new should point at next
                prev_nodes.push(curr_head.clone());
                next_nodes.push(next_node.clone());
                //attempt to go to next list if none then break
                list_i = match list_i.checked_sub(1) {
                    Some(i) => i,
                    None => break, //no list exists break loop
                };
            } else {
                return; //item already exists in the list 
            }
        }

        //reverse lists, we have been pushing to end for convience so they are in exact reverse order
        prev_nodes.reverse();
        next_nodes.reverse();

        //for all prev nodes set prev-> new_node
        for (list_i, node) in prev_nodes.iter().enumerate() {
            node.borrow_mut().set_next(new_head.clone(), list_i);
        }

        //we can just next_nodes for the new_head directyl
        new_head.borrow_mut().next_nodes = next_nodes;
    }

    /// Test if `value` exists in the skip list.
    ///
    /// # Arguments
    /// * `value` - The value to search for
    ///
    /// # Returns
    /// `true` if `value` is in the skip list, `false` otherwise
    pub fn exists(&self, value: &T) -> bool {
        return self.search(value).is_some();
    }

    /// Gets cloned `value` from skipList  
    ///
    /// # Arguments
    /// * `value` - The value to search for
    ///
    /// # Returns
    /// T `value` if `value` is in the skip list, None otherwise
    pub fn get(&self, value: &T) -> Option<T>
    where
        T: Clone,
    {
        match self.search(value) {
            Some(node) => return Some(node.borrow().value.clone()),
            None => return None,
        }
    }

    /// Deletes element with value 'T' from skiplist.
    ///
    /// # Arguments
    /// * `value` - The value to delete
    ///
    /// # Note
    /// This method will do nothing if element does not exist
    pub fn delete(&mut self, value: &T) {
        //deal with edge cases: deleting head or empty list
        match self.head.clone() {
            Some(head) => {
                //head is the value we are deleting
                if head.borrow().value == *value {
                    //get new head
                    let Some(new_head) = head.borrow_mut().next_nodes.first().cloned() else {
                        self.head = None; //There is only one element so table is now empty 
                        return;
                    };

                    //we need to promote the new head to point at all the lists
                    //start from list 1, list 0 is already done
                    for i in 1..head.borrow().next_nodes.len() {
                        let mut next_node = head.borrow().next_nodes[i].clone();
                        //if the head's next element is the new_head then we need to get the next element to avoid cycles
                        if next_node.borrow().value == new_head.borrow().value {
                            let Some(node) = next_node.borrow().next_nodes.get(i).cloned() else {
                                continue;
                            };
                            next_node = node;
                        }
                        //wire up new_head to point to list i
                        new_head.borrow_mut().set_next(next_node, i);
                    }

                    self.head = Some(new_head);
                    return;
                }
            }
            //list is empty
            None => return,
        };

        //for each list remove the next element and properly rewire lists
        loop {
            let Some((prev, list)) = self.search_prev(value) else {
                return; //exit loop once there is no prev value, search prev will search all lists
            };
            prev.borrow_mut().remove_next(list);
        }
    }

    ///Search for 'value'. This method is a convient wrapper for `search_prev()` with implements the optimal algorithm
    ///
    /// # Returns
    /// Node with `value` or None if not found
    fn search(&self, value: &T) -> Option<Head<T>> {
        match &self.head {
            Some(head) => {
                //head is the value
                if head.borrow().value == *value {
                    return Some(head.clone());
                }
            }
            //list is empty
            None => {
                return None; //empty list
            }
        };

        //use search prev
        let (prev, list) = self.search_prev(value)?;

        //this is safe since prev will be None if there is no next node(can't be a prev if there is no next)
        return Some(prev.borrow().next_nodes[list].clone());
    }

    ///Search for previous node to node that has 'value'. Recursively searches lists and returns the previous node in the highest skip list is exists in
    ///
    /// Returns
    /// Some(Head<T>, Usize) - tuple with previous node and list it was found
    /// None - Node not found, Node is head node,
    fn search_prev(&self, value: &T) -> Option<(Head<T>, usize)> {
        //get current root head
        let mut curr_head = self.head.clone()?;

        //check if head is value
        if curr_head.borrow().value == *value {
            return None;
        }

        //get total number of lists
        let mut list_i: usize = curr_head.borrow().next_nodes.len().saturating_sub(1);

        loop {
            //get next node
            let Some(next_node) = curr_head.borrow().next_nodes.get(list_i).cloned() else {
                //if no next node attempt to go to next list
                list_i = match list_i.checked_sub(1) {
                    Some(i) => i,
                    None => break, //no list exists break loop
                };
                continue;
            };

            if next_node.borrow().value < *value {
                //go to next node
                curr_head = next_node;
                continue;
            } else if next_node.borrow().value > *value {
                //attempt to go to next list if none then break
                list_i = match list_i.checked_sub(1) {
                    Some(i) => i,
                    None => break,
                };
            } else {
                //found item
                return Some((curr_head, list_i));
            }
        }

        //not in list
        return None;
    }

    ///literally a horrific terrible random solution
    /// I can only solve so many issues at once
    /// #TODO add a random crate or create good random
    fn coin_flip(&self) -> bool {
        let mut buf: [u8; 1] = [0];
        let mut f = File::open("/dev/urandom").expect("Eginea a kapot");
        f.read_exact(&mut buf).expect("ENGINGE A REALLY KAPOT");
        return buf[0] % 2 == 0;
    }
}

/// Formats the `SkipList` for display, showing each level with aligned columns.
/// Higher levels show only promoted nodes; absent nodes are rendered as blank
/// columns to preserve alignment with level 0.
///
/// VIBE CODED!!!! ALERT
/// # Author
/// claude
impl<T: Ord + fmt::Display> fmt::Display for SkipList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let head = match &self.head {
            None => return writeln!(f, "SkipList: empty"),
            Some(h) => h.clone(),
        };

        // --- Build master list from level 0 ---
        // Each entry is (display_string, Rc pointer) so we can later check
        // membership at higher levels via ptr_eq instead of string equality.
        let mut master: Vec<(String, Head<T>)> = Vec::new();
        {
            let mut curr = Some(head.clone());
            while let Some(rc) = curr {
                let next = rc.borrow().next_nodes.first().cloned();
                let val_str = rc.borrow().value.to_string(); // borrow dropped here
                master.push((val_str, rc)); // rc moved here safely
                curr = next;
            }
        }

        let max_level = head.borrow().next_nodes.len();
        let col_w = master.iter().map(|(s, _)| s.len()).max().unwrap_or(1) + 2; // padding
        let sep = "-".repeat(col_w * master.len() + 20);

        writeln!(
            f,
            "SkipList ({} levels, {} nodes):",
            max_level,
            master.len()
        )?;
        writeln!(f, "{}", sep)?;

        for level in (0..max_level).rev() {
            // Collect the Rc pointers present at this level, in order.
            let mut level_ptrs: Vec<Head<T>> = Vec::new();
            {
                let mut curr = Some(head.clone());
                while let Some(rc) = curr {
                    let next = rc.borrow().next_nodes.get(level).cloned();
                    level_ptrs.push(rc);
                    curr = next;
                }
            }
            let node_count = level_ptrs.len();

            write!(f, "L{}: ", level)?;

            // Walk the master list; print the value if this node is present at
            // `level` (checked by pointer identity), else print blanks.
            let mut level_iter = level_ptrs.iter();
            let mut next_in_level = level_iter.next();
            for (val_str, master_rc) in &master {
                let present = next_in_level
                    .map(|lrc| Rc::ptr_eq(lrc, master_rc))
                    .unwrap_or(false);

                if present {
                    write!(f, "{:<col_w$}", val_str)?;
                    next_in_level = level_iter.next();
                } else {
                    write!(f, "{:<col_w$}", "")?;
                }
            }
            writeln!(
                f,
                "-> None  ({} node{})",
                node_count,
                if node_count == 1 { "" } else { "s" }
            )?;
        }

        writeln!(f, "{}", sep)?;
        Ok(())
    }
}
