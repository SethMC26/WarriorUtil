use crate::utils::errors::UtilError;
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current time as milliseconds since UNIX epoch.
///
/// # Errors
/// * `UtilError::SystemTimeError` if the system clock is before UNIX epoch
pub fn current_time_millis() -> Result<u64, UtilError> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| UtilError::SystemTimeError(e))?
        .as_millis() as u64)
}
