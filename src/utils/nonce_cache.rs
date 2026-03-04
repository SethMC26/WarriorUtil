use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::Read;
use std::result::Result;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec::Vec;

use crate::utils::errors::UtilError;
use crate::utils::time::current_time_millis;

/// Thread safe Nonce Cache
/// `nonce_size` Number of bytes for each nonce value
/// `age_limit` Time in millis that each nonce is valid
pub struct NonceCache {
    //sizes  is from int(32 bit) from merrimackutil Noncecache
    nonce_size: u32,
    age_limit: u64,
    //cache is a shared mutable hashmap between threads
    cache: Arc<RwLock<HashMap<Vec<u8>, u64>>>,
}

impl NonceCache {
    /// Get random bytes from /dev/urandom this is a hacky quick fix we will need to find a good crypto crate later
    /// TODO update with crypto crate
    /// Claude generated function
    fn get_random_bytes(buf: &mut [u8]) -> Result<(), UtilError> {
        let mut f = File::open("/dev/urandom").map_err(|e| UtilError::IoError(e))?;
        f.read_exact(buf).map_err(|e| UtilError::IoError(e))?;
        Ok(())
    }

    /// Creates a new `NonceCache`.
    ///
    /// # Arguments
    /// * `nonce_size` - Expected size of each nonce in bytes
    /// * `age_limit`  - How long a nonce is valid in milliseconds
    pub fn new(nonce_size: u32, age_limit: u64) -> Self {
        NonceCache {
            nonce_size: nonce_size,
            age_limit: age_limit,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Adds a nonce to the cache.
    ///
    /// # Arguments
    /// * `nonce` - Byte slice of the nonce to add
    ///
    /// # Errors
    /// * `UtilError::InvalidInput` if nonce length does not match `nonce_size`
    /// * `UtilError::LockPoisoned` if the write lock has been poisoned
    /// * `UtilError::SystemTimeError` if the system clock fails
    pub fn add_nonce(&self, nonce: &[u8]) -> Result<(), UtilError> {
        //make sure nonce is correct size
        if nonce.len() != self.nonce_size as usize {
            return Err(UtilError::InvalidInput(format!(
                "Nonce must be {} bytes, got {} bytes",
                self.nonce_size,
                nonce.len()
            )));
        }

        let now = current_time_millis()?;

        let mut cache_guard = self
            .cache
            .write()
            .map_err(|e| UtilError::LockPoisoned(format!("Failed to acquire write lock: {}", e)))?;

        match cache_guard.entry(nonce.to_vec()) {
            Entry::Vacant(entry) => {
                entry.insert(now);
                return Ok(());
            }
            Entry::Occupied(mut entry) => {
                //only add nonce if entry is no longer valid
                if now - entry.get() > self.age_limit {
                    entry.insert(now);
                }
                //do not change nonce value if still valid - based on how merrimackutil works
                return Ok(());
            }
        }
    }

    /// Returns `true` if the nonce exists in the cache and has not expired.
    ///
    /// # Arguments
    /// * `nonce` - Byte slice of the nonce to check
    ///
    /// # Errors
    /// * `UtilError::InvalidInput` if nonce length does not match `nonce_size`
    /// * `UtilError::LockPoisoned` if the read lock has been poisoned
    /// * `UtilError::SystemTimeError` if the system clock fails
    pub fn contains_nonce(&self, nonce: &[u8]) -> Result<bool, UtilError> {
        if nonce.len() != self.nonce_size as usize {
            return Err(UtilError::InvalidInput(format!(
                "Nonce must be {} bytes, got {} bytes",
                self.nonce_size,
                nonce.len()
            )));
        }

        let cache_guard = self
            .cache
            .read()
            .map_err(|e| UtilError::LockPoisoned(format!("Failed to acquire read lock: {}", e)))?;

        match cache_guard.get(nonce) {
            None => Ok(false),
            Some(&time) => {
                let now = current_time_millis()?;
                //return if entry still valid
                Ok(now - time < self.age_limit)
            }
        }
    }

    /// Generate a nonce and return it as a vector of bytes
    /// # Returns
    /// * `Ok(Vec<u8>)` - A fresh nonce guaranteed not to be in the cache
    ///
    /// # Errors
    /// * `UtilError::IoError` if `/dev/urandom` cannot be read
    /// * `UtilError::LockPoisoned` if the write lock has been poisoned
    /// * `UtilError::SystemTimeError` if the system clock fails
    pub fn get_nonce(&self) -> Result<Vec<u8>, UtilError> {
        //generate nonce bytes
        let mut nonce_bytes = vec![0u8; self.nonce_size as usize];

        //loop until we get nonce not in cache
        loop {
            //fill nonce_bytes with a random bytes
            Self::get_random_bytes(&mut nonce_bytes)?;

            //get current time in millis
            let now: u64 = current_time_millis()?;

            //acquire write lock - if we get good nonce we need to write that value to cache
            let mut cache_guard = self.cache.write().map_err(|e| {
                UtilError::LockPoisoned(format!("Failed to acquire write lock: {}", e))
            })?;

            //get entry of nonce
            match cache_guard.entry(nonce_bytes.clone()) {
                //add entry if vacant
                Entry::Vacant(entry) => {
                    entry.insert(now);
                    return Ok(nonce_bytes);
                }
                //if occupied check if entry has expired if so add it
                Entry::Occupied(mut entry) => {
                    if now - entry.get() > self.age_limit {
                        entry.insert(now);
                        return Ok(nonce_bytes);
                    }
                    //entry not expired so we need another nonce
                }
            }
            //cache_guard goes out of scope and lock is dropped
        }
    }
}
