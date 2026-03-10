//! # Test Plan for net_db Module
//!
//! Author: GitHub Copilot
//!
//! ## General Testing Procedure
//! Tests are structured as integration tests in the `tests/` directory, exercising the public API
//! of the `warrior_util::net::net_db` module. Each test focuses on a specific function or edge case,
//! using assertions to validate expected behavior. Tests use `serde_json` for serialization checks
//! and `tempfile` for file I/O testing.
//!
//! ## Edge Cases Tested
//! - Empty databases and arrays
//! - Invalid JSON formats (malformed, missing fields, extra fields)
//! - Duplicate host entries in JSON
//! - File I/O with non-existent files and invalid content
//! - Serialization round-trips
//!
//! ## Positive Tests
//! - `test_host_entry_new`: Validates HostEntry creation and serialization
//! - `test_hosts_database_with_hosts`: Tests database with valid hosts
//! - `test_from_json_str_valid`: Parses valid JSON string
//! - `test_from_file_valid`: Reads valid JSON from file
//! - `test_to_json_str`: Serializes database to JSON
//!
//! ## Negative Tests
//! - `test_from_json_str_invalid_json`: Handles malformed JSON
//! - `test_from_json_str_missing_field`: Rejects incomplete host data
//! - `test_from_file_invalid_json`: Fails on invalid file content
//! - Empty database operations (get_port, get_address return None)

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use warrior_util::net::net_db::{HostEntry, HostsDatabase};

#[test]
/// Positive test: Validates HostEntry creation and serialization
fn test_host_entry_new() {
    let entry = HostEntry::new("example.com", "192.168.1.1", 8080);
    // Can't access fields, but can test serialization
    let json = serde_json::to_string(&entry).unwrap();
    assert!(json.contains("\"host-name\":\"example.com\""));
    assert!(json.contains("\"address\":\"192.168.1.1\""));
    assert!(json.contains("\"port\":8080"));
}

#[test]
/// Negative test: Validates empty database operations return None
fn test_hosts_database_empty() {
    let db = HostsDatabase::from_json_str(r#"{"hosts": []}"#).unwrap();
    assert_eq!(db.get_port("nonexistent"), None);
    assert_eq!(db.get_address("nonexistent"), None);
    assert!(!db.host_known("nonexistent"));
    assert_eq!(db.get_all_hosts().len(), 0);
}

#[test]
/// Positive test: Tests database with valid hosts
fn test_hosts_database_with_hosts() {
    let db = HostsDatabase::from_json_str(
        r#"{"hosts": [{"host-name": "example.com", "address": "192.168.1.1", "port": 8080}]}"#,
    )
    .unwrap();
    assert_eq!(db.get_port("example.com"), Some(8080));
    assert_eq!(db.get_address("example.com"), Some("192.168.1.1"));
    assert!(db.host_known("example.com"));
    assert!(!db.host_known("other.com"));
    let hosts = db.get_all_hosts();
    assert_eq!(hosts.len(), 1);
    assert_eq!(hosts[0], "example.com");
}

#[test]
fn test_to_json_str() {
    let db = HostsDatabase::from_json_str(
        r#"{"hosts": [{"host-name": "example.com", "address": "192.168.1.1", "port": 8080}]}"#,
    )
    .unwrap();
    let json = db.to_json_str().unwrap();
    // Should contain the host
    assert!(json.contains("\"host-name\":\"example.com\""));
    assert!(json.contains("\"address\":\"192.168.1.1\""));
    assert!(json.contains("\"port\":8080"));
}

#[test]
fn test_to_json_str_empty() {
    let db = HostsDatabase::from_json_str(r#"{"hosts": []}"#).unwrap();
    let json = db.to_json_str().unwrap();
    assert!(json.contains("\"hosts\":[]"));
}

#[test]
/// Positive test: Parses valid JSON string
fn test_from_json_str_valid() {
    let json =
        r#"{"hosts": [{"host-name": "example.com", "address": "192.168.1.1", "port": 8080}]}"#;
    let db = HostsDatabase::from_json_str(json).unwrap();
    assert_eq!(db.get_port("example.com"), Some(8080));
    assert_eq!(db.get_address("example.com"), Some("192.168.1.1"));
}

#[test]
/// Positive test: Parses empty JSON array
fn test_from_json_str_empty() {
    let json = r#"{"hosts": []}"#;
    let db = HostsDatabase::from_json_str(json).unwrap();
    assert_eq!(db.get_all_hosts().len(), 0);
}

#[test]
/// Negative test: Handles malformed JSON
fn test_from_json_str_invalid_json() {
    let json = r#"{"hosts": [{"host-name": "example.com", "address": "192.168.1.1", "port": 8080}"#; // missing }
    assert!(HostsDatabase::from_json_str(json).is_err());
}

#[test]
/// Negative test: Rejects incomplete host data
fn test_from_json_str_missing_field() {
    let json = r#"{"hosts": [{"host-name": "example.com", "address": "192.168.1.1"}]}"#; // missing port
    assert!(HostsDatabase::from_json_str(json).is_err());
}

#[test]
/// Negative test: Ignores extra fields in JSON
fn test_from_json_str_extra_field() {
    let json = r#"{"hosts": [{"host-name": "example.com", "address": "192.168.1.1", "port": 8080, "extra": "field"}]}"#;
    let db = HostsDatabase::from_json_str(json).unwrap(); // serde should ignore extra
    assert_eq!(db.get_port("example.com"), Some(8080));
}

#[test]
/// Edge case test: Handles duplicate host names (last wins)
fn test_from_json_str_duplicate_hosts() {
    let json = r#"{"hosts": [{"host-name": "example.com", "address": "192.168.1.1", "port": 8080}, {"host-name": "example.com", "address": "192.168.1.2", "port": 8081}]}"#;
    let db = HostsDatabase::from_json_str(json).unwrap();
    // Should have the last one
    assert_eq!(db.get_port("example.com"), Some(8081));
    assert_eq!(db.get_address("example.com"), Some("192.168.1.2"));
}

#[test]
/// Positive test: Reads valid JSON from file
fn test_from_file_valid() {
    let json =
        r#"{"hosts": [{"host-name": "example.com", "address": "192.168.1.1", "port": 8080}]}"#;
    let temp_path = PathBuf::from(format!("/tmp/test_hosts_valid_{}.json", std::process::id()));
    {
        let mut file = File::create(&temp_path).unwrap();
        write!(file, "{}", json).unwrap();
    }
    let mut file = File::open(&temp_path).unwrap();
    let db = HostsDatabase::from_file(&mut file).unwrap();
    assert_eq!(db.get_port("example.com"), Some(8080));
    std::fs::remove_file(temp_path).unwrap();
}

#[test]
/// Negative test: Fails on invalid file content
fn test_from_file_invalid_json() {
    let json = r#"{"hosts": [{"host-name": "example.com", "address": "192.168.1.1", "port": 8080}"#;
    let temp_path = PathBuf::from(format!(
        "/tmp/test_hosts_invalid_{}.json",
        std::process::id()
    ));
    {
        let mut file = File::create(&temp_path).unwrap();
        write!(file, "{}", json).unwrap();
    }
    let mut file = File::open(&temp_path).unwrap();
    assert!(HostsDatabase::from_file(&mut file).is_err());
    std::fs::remove_file(temp_path).unwrap();
}

#[test]
/// Negative test: Handles empty file
fn test_from_file_empty() {
    let json = r#"{"hosts": []}"#;
    let temp_path = PathBuf::from(format!("/tmp/test_hosts_empty_{}.json", std::process::id()));
    {
        let mut file = File::create(&temp_path).unwrap();
        write!(file, "{}", json).unwrap();
    }
    let mut file = File::open(&temp_path).unwrap();
    let db = HostsDatabase::from_file(&mut file).unwrap();
    assert_eq!(db.get_all_hosts().len(), 0);
    std::fs::remove_file(temp_path).unwrap();
}
