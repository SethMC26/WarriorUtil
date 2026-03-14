// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

use std::cell::RefCell;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

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
}

/// SkipList collection on average O(lg(n)) insert and search
/// We can insert any element that implements Ord(ordering) which is similar to comparable in java
/// `head` - Current Head of the skip list which points to all levels
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
        if &root_head.borrow().value > &new_head.borrow().value {
            let mut new_node = new_head.borrow_mut();
            //have new node point to each list 
            for node in &root_head.borrow().next_nodes {
                new_node.next_nodes.push(node.clone());
            }
            //add root head back to list 
            new_node.next_nodes[0] = root_head.clone();
            self.head = Some(new_head.clone());
            return;
        }

        //insert into root list(list 0)
        Self::insert_into_list(root_head.clone(), new_head.clone(), 0);

        //while we get "heads" promote node to the next
        let mut i: usize = 1;
        while Self::coin_flip() {
            Self::insert_into_list(root_head.clone(), new_head.clone(), i);
            i += 1;
        }
    }

    /// Inserts `new_head` into the linked list at the given level, maintaining sorted order.
    ///
    /// Traverses the list at level `list` starting from `head` until it finds the
    /// correct position, then wires up the forward pointers. Duplicate values are
    /// silently ignored.
    ///
    /// # Arguments
    /// * `head` - The head node to start traversal from, always the leftmost node
    /// * `new_head` - The node to insert
    /// * `list` - The level to insert into
    fn insert_into_list(head: Head<T>, new_head: Head<T>, list: usize) {
        let mut curr_head = head;
        //now we must search the skip list 0 to insert the new element
        loop {
            //#TODO optimize insert to use better search instead of trivial search

            //get next node, use block expression so curr_head borrow is dropped at the end of that scope
            let next_head: Option<Head<T>> = { curr_head.borrow().next_nodes.get(list).cloned() };

            //need to use if statement here instead of match since borrow will be dropped after condition executes
            if let Some(next_head) = next_head {
                if &next_head.borrow().value > &new_head.borrow().value {
                    //new value should be inserted here 
                    //wire up forward pointers, set curr_head to point ot new node and new node to point at next node
                    curr_head.borrow_mut().set_next(new_head.clone(), list);
                    new_head.borrow_mut().set_next(next_head.clone(), list);
                    return;
                } else if &next_head.borrow().value == &new_head.borrow().value {
                    return; //value aready in skip list ya dummy 
                } else {
                    curr_head = next_head.clone(); // go to loop iteration 
                }
            } else {
                //last iteration since next_head does not exist
                curr_head.borrow_mut().set_next(new_head, list);
                return;
            }
        }
    }

    ///literally a horrific terrible random solution
    /// I can only solve so many issues at once
    /// #TODO add a random crate or create good random
    fn coin_flip() -> bool {
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

        // Collect all values at level 0 in order — this is the master list.
        // All higher levels are subsets of this list.
        let mut all_values: Vec<String> = Vec::new();
        {
            let mut curr = Some(head.clone());
            while let Some(rc) = curr {
                let next = {
                    let node = rc.borrow();
                    all_values.push(node.value.to_string());
                    node.next_nodes.get(0).cloned()
                };
                curr = next;
            }
        }

        // Compute column width from the widest value string so all columns align.
        let width = all_values.iter().map(|v| v.len()).max().unwrap_or(1);
        let arrow = " -> ";
        let col_width = width + arrow.len();
        let blank = " ".repeat(col_width);

        // Level count is the number of next_nodes on the head since head
        // participates in every level by definition.
        let max_level = head.borrow().next_nodes.len();

        writeln!(f, "SkipList ({} levels, {} nodes):", max_level, all_values.len())?;
        writeln!(f, "{}", "-".repeat(col_width * all_values.len() + 6))?;

        // Print levels from highest to lowest so the sparsest list is on top.
        for i in (0..max_level).rev() {

            // Collect values present at this level by traversing from head.
            let mut level_values: Vec<String> = Vec::new();
            let mut level_count = 0;
            {
                let mut curr = Some(head.clone());
                while let Some(rc) = curr {
                    let next = {
                        let node = rc.borrow();
                        level_values.push(node.value.to_string());
                        level_count += 1;
                        node.next_nodes.get(i).cloned()
                    };
                    curr = next;
                }
            }

            write!(f, "L{}: ", i)?;

            // Walk the master list (level 0), printing the value if it exists at
            // this level, or a blank column if it was not promoted.
            let mut level_iter = level_values.iter().peekable();
            for val in &all_values {
                if level_iter.peek().map(|v| *v == val) == Some(true) {
                    write!(f, "{:>width$}{}", val, arrow, width = width)?;
                    level_iter.next();
                } else {
                    write!(f, "{}", blank)?;
                }
            }
            writeln!(f, "None  ({} nodes)", level_count)?;
        }

        writeln!(f, "{}", "-".repeat(col_width * all_values.len() + 6))?;
        writeln!(f, "L0 values: {:?}", all_values)?;

        Ok(())
    }
}