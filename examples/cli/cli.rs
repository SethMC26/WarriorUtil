use warrior_util::utils::cli;
use warrior_util::utils::cli::LongOp;

fn main() {
    let ops = vec![
        LongOp::new("h", "help", "Display the help.").has_arg(false),
        LongOp::new("p", "port", "Port to use."),
        LongOp::new("c", "config", "Config file to use."),
    ];

    let op_map = cli::get_op_map(&ops).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        eprintln!("{}", cli::options_string(&ops));
        std::process::exit(1);
    });

    if op_map.contains_key("help") {
        println!("{}", cli::options_string(&ops));
        return;
    }

    let port: u32 = op_map
        .get("port")
        .ok_or("port is required")
        .and_then(|p| p.parse().map_err(|_| "port must be a number"))
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            eprintln!("{}", cli::options_string(&ops));
            std::process::exit(1);
        });

    let config_file = op_map
        .get("config")
        .map(|s| s.as_str())
        .unwrap_or("config.json");

    println!("Port: {} Config: {}", port, config_file);
}
