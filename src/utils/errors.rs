use std::fmt;
use std::time::SystemTimeError;

/// All possible errors from warrior-util
/// Created by claude
#[derive(Debug)]
pub enum UtilError {
    /// RwLock or Mutex was poisoned by a panicking thread
    LockPoisoned(String),
    /// IO error — file read or write failed
    IoError(std::io::Error),
    /// System clock error — time went backwards
    SystemTimeError(SystemTimeError),
    /// Invalid input provided by caller
    InvalidInput(String),
}

/// Display — human readable messages for each error
impl fmt::Display for UtilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UtilError::LockPoisoned(msg) => write!(f, "lock poisoned by panicking thread: {}", msg),

            UtilError::IoError(e) => write!(f, "IO error: {}", e),

            UtilError::SystemTimeError(e) => write!(f, "system clock error: {}", e),

            UtilError::InvalidInput(msg) => write!(f, "invalid input: {}", msg),
        }
    }
}

/// Error trait — body is empty because Debug and Display
/// satisfy all requirements
impl std::error::Error for UtilError {
    /// Expose underlying error source for error chaining
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            UtilError::IoError(e) => Some(e),
            UtilError::SystemTimeError(e) => Some(e),
            _ => None,
        }
    }
}
