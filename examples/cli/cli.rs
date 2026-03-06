use warrior_util::utils::cli;
use warrior_util::utils::cli::LongOp;

fn main() {
    //create options for cli, first is short option, second is long option and third is usage information
    //we can set the option to not have an arg by chaining has.arg() by default has_arg is set to true
    let help_op = LongOp::new("h", "help", "Display the help.").has_arg(false);
    let port_op = LongOp::new("p", "port", "Port to use.");
    let config_op = LongOp::new("c", "config", "Config file to use.");

    //notice that vec now has ownership of these variables
    let ops = vec![help_op, port_op, config_op];

    //if we want explicit names we can borrow from vector like let help_key = &vec[0] to borrow it

    //lets make the usage string
    let mut usage: String = String::from(
        "Usage:\n  example --config <config> --port <port>\n  example --port <port>\n  example --help\n",
    );
    //add options using helper method
    usage.push_str(&cli::options_string(&ops));

    //This is where the magic happens this returns a map of the arguments for each option
    //We will get an error here if arguments are not valid such as --badflag test will give an error
    let op_map = cli::get_op_map(&ops).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        eprintln!("{}", usage);
        std::process::exit(1);
    });

    //we can check if an argument is passed by checking if it contains the key
    //note we need to use it from the vec since the vector how has ownership over config_op
    if op_map.contains_key(&ops[0]) {
        println!("{}", usage);
        return;
    }

    //here is how we can check a required argument
    //check if port is passed
    let port: u32 = op_map
        //Map will return None if argument was not provided
        .get(&ops[1])
        //Turn option to result(None gets mapped into error)
        .ok_or("port is required")
        //if not error attempt to parse port as number infers type from port: u32 and if error map error to string type error
        .and_then(|p| p.parse().map_err(|_| "port must be a number"))
        //if error handle it with closure
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            eprintln!("{}", usage);
            std::process::exit(1);
        });

    //here we set a default argument if none provided
    let config_file = op_map
        .get(&ops[2])
        .map(|s| s.as_str())
        //if none(not provided) set default value
        .unwrap_or("config.json");

    //print config
    println!("Port: {} Config: {}", port, config_file);
}
