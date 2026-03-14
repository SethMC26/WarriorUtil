use warrior_util::collections::skip_list::SkipList;

#[test]
fn test_insert_empty() {
    let mut list = SkipList::new();
    list.insert(5);
}

#[test]
fn test_insert_middle_then_end() {
    let mut list = SkipList::new();
    list.insert(10);
    list.insert(20);
    list.insert(15); // middle
    list.insert(25); // end
}

#[test]
fn test_insert_middle_then_beginning() {
    let mut list = SkipList::new();
    list.insert(10);
    list.insert(20);
    list.insert(15); // middle
    list.insert(5); // beginning
}

#[test]
fn test_insert_middle_then_both_ends() {
    let mut list = SkipList::new();
    list.insert(10);
    list.insert(20);
    list.insert(15); // middle
    list.insert(25); // end
    list.insert(5); // beginning
}

#[test]
fn test_insert_multiple_middle() {
    let mut list = SkipList::new();
    list.insert(10);
    list.insert(40);
    list.insert(20); // middle
    list.insert(30); // middle
    list.insert(25); // middle between middles
    list.insert(50); // end
    list.insert(5); // beginning
}

#[test]
fn test_insert_adjacent_middle() {
    let mut list = SkipList::new();
    list.insert(10);
    list.insert(30);
    list.insert(20); // middle
    list.insert(21); // adjacent to middle, tests pointer wiring between close values
    list.insert(19); // just before middle
}
