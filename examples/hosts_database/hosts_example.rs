// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

use std::fs::File;
use warrior_util::net::net_db::HostsDatabase;

fn main() {
    // Basic input with common file associated with many projects in Networking at Merrimack College
    //r# allows us to use raw string literals and ignores escape characters, Neat!
    let json_input = r#"
    {
        "hosts": [
            {
                "host-name": "google.com",
                "address": "8.8.8.8",
                "port": 443
            },
            {
                "host-name": "localhost",
                "address": "127.0.0.1",
                "port": 8080
            }
        ]
    }
    "#;

    // Deserialize
    let db = HostsDatabase::from_json_str(json_input)
        .expect("Cannot convert JSON to host Database, Panic!");

    // Use helper methods
    println!("Known hosts: {:?}", db.get_all_hosts());
    println!("google.com port: {:?}", db.get_port("google.com"));
    println!("localhost address: {:?}", db.get_address("localhost"));

    //helpers return an Option type which is either None or Some value
    let port: Option<u16> = db.get_port("badhost.com");

    //here is how we can use options
    match port {
        Some(x) => {
            println!("Good Port from host {}", x);
        }
        None => {
            //handle error here
            println!("Unknown host! Cannot get port")
        }
    }

    //or we can assign a default value in this case set port to 0 if unknown
    let port = db.get_port("badhost").unwrap_or(0);

    println!("Badhost port {}", port);
    // Serialize back into string
    let serialized = db.to_json_str().expect("Cannot convert to JSON, panic!");

    println!("\nSerialized again:\n{}", serialized);

    //similar to before instead of option type we also have some Result types for to_json_str and from_json_str
    //Result types return an the value or an Error we can use much the same methods as before

    let bad_keyname = r#"
    {"hosts":[ {"gotcorrupted": 8080, "host-name": "localhost", "address": "127.0.0.1"} ]
    "#;

    let bad_key_value = r#"
    {"hosts":[ {"port": "8080", "host-name": "localhost", "address": "127.0.0.1"} ]
    "#;

    let bad_json_value = r#"
    {"hosts" _ [ {"port": 8080, "host-name": "localhost", "address": "127.0.0.1"} ]
    "#;

    println!("Attempt to build hostdatabase from bad JSON");
    println!(
        "Bad keywords: {:?}",
        HostsDatabase::from_json_str(bad_keyname)
    );
    println!(
        "Bad type from key: {:?}",
        HostsDatabase::from_json_str(bad_key_value)
    );
    println!(
        "Bad JSON: {:?}",
        HostsDatabase::from_json_str(bad_json_value)
    );

    //here is one way to deal with errors
    let db = HostsDatabase::from_json_str(bad_keyname);

    if db.is_err() {
        //deal with error
        println!("Got error while parsing host database...")
    }
    //depending on how we want to handle errors rust provides some nice ways to reduce boilerplate code

    //we can also create the host database from a file
    let mut file = File::open("examples/hosts_database/hosts.json").expect("File exists");
    let db = HostsDatabase::from_file(&mut file).expect("Good JSON format");
    println!("db from file {:?}", db);
}
