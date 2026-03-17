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
