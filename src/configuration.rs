extern crate serde_json;

use std::path::Path;
use std::fs::File;
use std::{env,process};
use std::io::{stderr,BufReader,Write};

pub fn load() -> serde_json::Value {
    let home_dir_path_buffer = match env::home_dir() {
        Some(path) => path,
        None => {
            writeln!(
                stderr(),
                "Couldn't find home directory."
            ).unwrap();
            process::exit(1);
        }
    };

    let home_dir_path = home_dir_path_buffer.to_str().unwrap();
    let config_file_path = format!("{}/.config/osxnotifysounds/config.json", home_dir_path);

    if !Path::new(&config_file_path).exists() {
        writeln!(
            stderr(),
            "You don't have a configuration set at {}.\n\
            Make sure that's in place first, so I know which sounds to use for which notification.",
            config_file_path
        ).unwrap();
        process::exit(1);
    }

    let file = File::open(config_file_path).expect("Json file not found.");

    let file_reader = BufReader::new(file);
    serde_json::from_reader(file_reader)
        .expect("Couldn't parse json file.  Validate with json linter to confirm.")
}