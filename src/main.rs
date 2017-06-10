extern crate serde_json;
extern crate rusqlite;

use std::{env, thread, time};
use std::process::Command;
use serde_json::Value;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use rusqlite::Connection;

mod notificationcenter;

fn main() {
    let mut app_notes: Vec<(u32, serde_json::Value)> = Vec::new();

    let path_buffer = match env::home_dir() {
        Some(path) => path,
        None => {
            println!("Couldn't find home directory.");
            std::process::exit(1);
        }
    };

    let home_dir_path = path_buffer.to_str().unwrap();
    let config_file_path = format!("{}/.config/osxnotifysounds/config.json", home_dir_path);

    if !Path::new(&config_file_path).exists() {
        println!("You don't have a configuration set at {}.\n\
                 Make sure that's in place first, so I know which sounds to use for
                 which notification.",
                 config_file_path);
        std::process::exit(1);
    }

    let file = File::open(config_file_path).expect("Json file not found.");

    let file_reader = BufReader::new(file);
    let config_json: Value =
        serde_json::from_reader(file_reader)
            .expect("Couldn't parse json file.  Validate with json linter to confirm.");

    let tmpdir = env::var("TMPDIR").expect("could not read TMPDIR env variable");
    let notificationcenter_path = format!("{}../0/com.apple.notificationcenter/db/db", tmpdir);
    let conn = Connection::open(notificationcenter_path).expect("could not open database");

    let app_iter = config_json
        .get("applications")
        .unwrap()
        .as_array()
        .expect("applications value is not an array")
        .iter();

    for app in app_iter {
        for (_, app_details) in app.as_object().unwrap().iter() {
            let newest_note =
                notificationcenter::get_newest_note(
                    app_details
                    .get("app_id")
                    .unwrap()
                    .as_u64()
                    .unwrap() as u32,
                    &conn
            );
            app_notes.push((newest_note, app_details.clone()));
        }
    }

    loop {
        for app_entry in &mut app_notes {
            let app_id = app_entry.1.get("app_id").unwrap().as_u64().unwrap() as u32;
            let app_data = notificationcenter::perform_db_lookup(app_entry.0,app_id, &conn);

            for data in app_data {
                match data {
                    Ok(note_data) => {
                        let (note_id, encoded_data): (u32, Option<Vec<u8>>) = note_data;
                        let encoded_data = encoded_data.unwrap();
                        let encoded_data = String::from_utf8_lossy(&encoded_data);

                        let note_iter =
                            app_entry.1["notification_details"].as_object() .unwrap().iter();

                        for (_, notification_details) in note_iter {
                            let look_for = notification_details["lookfor"].clone();
                            let sound = notification_details["sound"].clone();

                            if look_for
                                .as_array()
                                .expect("'lookfor' json is not an array")
                                .iter()
                                .any(|data| encoded_data.contains(data.as_str().unwrap())) {
                                Command::new("sh")
                                    .arg("-c")
                                    .arg(&format!("afplay {}", sound))
                                    .output()
                                    .expect("afplay failed??");
                                }
                        }

                        app_entry.0 = note_id;
                    }
                    Err(_) => continue,
                };
            }

            thread::sleep(time::Duration::from_secs(1));
        }
    }
}
