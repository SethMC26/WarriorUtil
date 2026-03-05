use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

//TODO update with more merrimack Util like serialization
#[derive(Serialize, Deserialize, Debug)]
/// Represents a single host entry.
///
/// Serialized field names follow Merrimack-style JSON conventions:
/// - `host_name` → `"host-name"`
/// - `address`
/// - `port`
pub struct HostEntry {
    port: u16,
    #[serde(rename = "host-name")]
    host_name: String,
    //for now we are using epic json macros to do all our dirty json work, we can rename keys this way
    address: String,
}
impl HostEntry {
    ///Returns a HostEntry with given port, host_name and address
    /// # Arguments
    /// * `host_name`
    /// * `address`
    /// * `port`
    pub fn new(host_name: &str, address: &str, port: u16) -> Self {
        HostEntry {
            host_name: host_name.to_string(),
            port: port,
            address: address.to_string(),
        }
    }
}
//TODO update with more merrimack Util like serialization
#[derive(Serialize, Deserialize, Debug)]
/// Simple hosts database modeled after MerrimackUtil.
///
/// ⚠ Note:
/// The original MerrimackUtil JSON format uses:
///
/// `{ "hosts": [ { "address": "...", "port": ..., "host-name": "..." } ] }`
///
/// This implementation instead stores hosts as a map:
///
/// `{ "hosts": { "example.com": { "address": "...", "port": ..., "host-name": "..." } } }`
///
/// Use `to_json_str()` and `from_json_str()` if you need the original array-based structure.
pub struct HostsDatabase {
    #[serde(rename = "hosts")]
    host_map: HashMap<String, HostEntry>,
}

impl HostsDatabase {
    ///Returns the port of a given host_name, None if hostname is not in database
    /// # Arguments
    /// * 'host_name' - String slice of hostname
    pub fn get_port(&self, host_name: &str) -> Option<u16> {
        self.host_map.get(host_name).map(|e| e.port)
    }

    ///Returns the address of a given host_name, None if hostname is not in the database
    /// # Arguments
    /// * 'host_name' - String slice of hostname
    pub fn get_address(&self, host_name: &str) -> Option<&str> {
        self.host_map
            .get(host_name)
            .map(|e: &HostEntry| e.address.as_str())
    }

    ///Returns the true if the host in the database false otherwise
    /// # Arguments
    /// * `host_name` - String slice of hostname
    pub fn host_known(&self, host_name: &str) -> bool {
        self.host_map.contains_key(host_name)
    }

    ///Returns a `Vec` with all the known hostnames
    pub fn get_all_hosts(&self) -> Vec<&str> {
        //get all the keys, turn them into string slice and collect the results(rust casts into Vec)
        self.host_map.keys().map(|e| e.as_str()).collect()
    }

    /// Returns Serialized database as a JSON string into the format from MerrimackUtil
    ///
    /// `{ "hosts": [ { "host-name": "...", "address": "...", "port": ... }, ...] }`
    pub fn to_json_str(&self) -> Result<String, Error> {
        let values: Vec<&HostEntry> = self.host_map.values().collect();
        serde_json::to_string(&serde_json::json!({
            "hosts" : values
        }))
    }

    /// Deserializes the format used in CS classes at Merrimack College for hosts into a `HostsDatabase`.
    ///
    /// Expects JSON of the form:
    /// `{ "hosts": [ { "host-name": "...", "address": "...", "port": ... }, ...] }`
    /// # Arguments
    /// * `json_str` String slice to turn into Hosts Database
    pub fn from_json_str(json_str: &str) -> Result<Self, Error> {
        ///helper struct to turn the hosts array into a hash map using serde macros
        #[derive(Deserialize)]
        struct HostsArray {
            hosts: Vec<HostEntry>,
        }

        // Deserialize into struct(type safe yayyy)
        let hosts_array: HostsArray = serde_json::from_str(json_str)?;
        //turn the array into a map
        let mut map: HashMap<String, HostEntry> = HashMap::new();
        for host in hosts_array.hosts {
            map.insert(host.host_name.clone(), host);
        }

        Ok(HostsDatabase { host_map: map })
    }

    ///Create a HostDatabase from a file
    ///
    ///Expects JSON of the form:
    /// `{ "hosts": [ { "host-name": "...", "address": "...", "port": ... }, ...] }`
    ///
    pub fn from_file(file: &mut File) -> Result<Self, Error> {
        let mut json_str = String::new();
        //read contents of file into string
        let _ = file.read_to_string(&mut json_str).map_err(Error::io)?;
        //turn string into json of hostDB
        Self::from_json_str(json_str.as_str())
    }
}
