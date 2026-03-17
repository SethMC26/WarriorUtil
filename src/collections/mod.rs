pub mod skip_list;

/// Triggers a panic in debug builds with the given message.
/// # Note
/// Macro simply wraps `debug_assert(false, $msg)``
#[macro_export]
macro_rules! debug_panic {
    ($msg: expr) => {
        debug_assert!(false, $msg)
    };
}

/// Creates a [`SkipList`] containing the given values. SkipList literal expression
///
/// # Examples
///
/// ```ignore
/// let list = skip_list![1, 2, 3];
/// let list SkipList<i32> = skip_list![];
/// ```
///
#[macro_export]
macro_rules! skip_list {
    ($($val:expr),*) => {{
        let mut skip_list = $crate::collections::skip_list::SkipList::new();
        //insert a val for each time that val has appeared
        $(skip_list.insert($val);)*
        skip_list
    }}
}
