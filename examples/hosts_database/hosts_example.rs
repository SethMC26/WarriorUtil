use warrior_util::net::net_db::HostsDatabase;

fn main() {
    // Merrimack-style JSON input
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
    let db = HostsDatabase::from_json_str(json_input).unwrap();

    // Use helper methods
    println!("Known hosts: {:?}", db.get_all_hosts());
    println!("google.com port: {:?}", db.get_port("google.com"));
    println!("localhost address: {:?}", db.get_address("localhost"));

    // Serialize back
    let serialized = db.to_json_str().unwrap();

    println!("\nSerialized again:\n{}", serialized);
}