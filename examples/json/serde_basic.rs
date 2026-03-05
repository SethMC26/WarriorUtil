use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

fn main() {
    //we can use some cool tricks with serde_json to quick create json in a type safe way
    //we can define structs to have types and values for json values

    //let us recreate this example https://github.com/stuetzlec/merrimack_cs_examples/tree/main/merrimackutil/json/src

    //first the macros will add the impl methods for Serialize and Deserialize automagically
    //this is done as a precompiler step, a fancy(extremely powerful) string replacement
    #[derive(Serialize, Deserialize, Debug)]
    struct Configuration {
        //by default the macro above will use the names of fields as keynames in json
        //we can change this using the rename macro
        //serde also provides #[serde(rename_all = "camelCase")], we could also rename_all to other cases like "kebab-case"
        #[serde(rename = "host")]
        port_number: u16,
        #[serde(rename = "logFile")]
        log_file: String,
        #[serde(rename = "dataDir")]
        data_dir: String,
    }

    let json_str = r#"
        {"host": 4000, "logFile": "log.txt", "dataDir": "my/file/location"}
    "#;

    //now we can easily the json from a string
    //serde will use attempt to take json and call Deserialize on Configuration, notice we need to specify the type here explicitly since it cannot be infered
    let conf: Configuration = serde_json::from_str(json_str).expect("Valid json");
    println!(
        "Host {} location {}{}",
        conf.port_number, conf.data_dir, conf.log_file
    );

    //but wait if our json is invalid we panic! how can we handle these errors?
    let bad_json = r#"
        {"host": "4000", "logFile": "log.txt", "dataDir": "my/file/location"}
    "#;

    let _conf: Configuration = match serde_json::from_str(json_str) {
        Ok(config) => config,
        Err(e) => {
            //handle error lets set default to please compiler but we could also panic!
            println!("Bad json for configuration file got error: {}", e);
            Configuration {
                port_number: 0000,
                log_file: "defaultfile".into(),
                data_dir: "default/dir".into(),
            }
        }
    };
    //we can also do this in the rustic way using a closure
    let _conf: Configuration = serde_json::from_str(bad_json).unwrap_or_else(|e| {
        println!("Bad json for configuration file got error: {}", e);
        Configuration {
            port_number: 0000,
            log_file: "defaultfile".into(),
            data_dir: "default/dir".into(),
        }
    });

    //also notice that this is type safe! the derived serialized and deserialize will check the types and return an error if there is a type mismatch

    //we may want to get it from a file as well for this we can use the reader option
    //we can handle errors similar to before but for the example we will ignore errors and crash instead
    let file = File::open("examples/json/configuration.json").expect("Valid file path");
    let reader = BufReader::new(file);
    let conf: Configuration = serde_json::from_reader(reader).expect("Valid json");

    //we may want to write a file
    let file = File::create("examples/json/conf_pretty.json").expect("Can create file");
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &conf).expect("JSON is valid");
}
