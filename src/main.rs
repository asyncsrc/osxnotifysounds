extern crate rusqlite;
extern crate clap;

use std::{thread, time};
use std::process::Command;
use rusqlite::Connection;
use std::env;

use clap::{Arg, App};

mod notificationcenter;
mod configuration;

fn main() {

    let matches = App::new("osxnotifysounds")
                          .version("1.0")
                          .author("Joseph Gimenez <joseph.gimenez@snagajob.com>")
                          .about("Define custom sounds for Notification Center Alerts")
                          .arg(Arg::with_name("APP_NAME")
                               .short("a")
                               .help("Lookup app_id for application")
                               .takes_value(true))
                          .get_matches();

    let config_json = configuration::load();

    let tmpdir = env::var("TMPDIR").expect("could not read TMPDIR env variable");
    let notificationcenter_path = format!("{}../0/com.apple.notificationcenter/db/db", tmpdir);
    let conn = Connection::open(notificationcenter_path).expect("could not open database");

    if let Some(app_name) = matches.value_of("APP_NAME") {
        let results = notificationcenter::lookup_app_id(app_name, &conn);
        for app_id_result in &results {
            if let Ok(ref app_result) = *app_id_result {
                    println!("Matched application: {} -- app_id: {}",
                    app_result.bundleid,
                    app_result.app_id)
            }
        }
        std::process::exit(0);
    }

    let mut app_notes = notificationcenter::populate_app_notes(&config_json, &conn);

    loop {
        for app_entry in &mut app_notes {
            let app_id = app_entry.1.get("app_id").unwrap().as_u64().unwrap() as u32;
            let app_data = notificationcenter::get_newest_alerts_for_app(app_entry.0,app_id, &conn);

            for data in app_data {
                match data {
                    Ok(note_data) => {
                        let notification_data = note_data;
                        let encoded_data = notification_data.encoded_data;
                        let encoded_data = String::from_utf8_lossy(&encoded_data);

                        let note_iter =
                            app_entry.1["notification_details"].as_object() .unwrap().iter();

                        for (_, notification_details) in note_iter {
                            let look_for = notification_details["look_for"].clone();
                            let sound = notification_details["sound"].clone();

                            // iterate through each look_for item and see if any are found in alert text
                            if look_for
                                .as_array()
                                .expect("'look_for' json is not an array")
                                .iter()
                                .any(|data| encoded_data.contains(data.as_str().unwrap())) {
                                    Command::new("sh")
                                    .arg("-c")
                                    .arg(&format!("afplay {}", sound))
                                    .output()
                                    .expect("afplay failed??");
                                }
                        }
                        // update our latest note counter, so don't play custom sounds on old alerts
                        app_entry.0 = notification_data.note_id;
                    }
                    Err(_) => continue,
                };
            }

            thread::sleep(time::Duration::from_secs(1));
        }
    }
}
