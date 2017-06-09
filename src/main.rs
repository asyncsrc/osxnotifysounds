extern crate rusqlite;
extern crate serde_json;

use rusqlite::Connection;
use std::{env, thread, time};
use std::process::Command;
use serde_json::Value;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

const SLACK_APP_ID: u32 = 25;

fn get_newest_note(conn: &rusqlite::Connection) -> u32 {
    let newest_entry =
        match conn.query_row(
            &format!(
                "SELECT note_id from notifications where app_id = {} order by note_id desc limit 1",
                SLACK_APP_ID), &[], |row| row.get(0)) {
                    Ok(entry) => {
                        println!("Starting from sqlite notification entry: {}", entry);
                        entry
                    },
                    Err(err) => {
                        writeln!(
                            &mut std::io::stderr(),
                            "Error occurred: {}.  Waiting for first event for app: {}",
                            err,
                            SLACK_APP_ID).unwrap();
                        0
                    }
        };
    newest_entry
}

fn perform_db_lookup<X, Y>(newest_note: u32,
                           conn: &rusqlite::Connection)
                           -> Vec<std::result::Result<(X, Y), rusqlite::Error>>
    where X: rusqlite::types::FromSql,
          Y: rusqlite::types::FromSql
{
    let mut stmt = conn.prepare(
        &format!(
            "SELECT note_id, encoded_data from notifications where app_id = {} and note_id > {} order by note_id",
            SLACK_APP_ID,
            newest_note),
    ).expect("Could not prepare SQL select statement.");

    let note_iter = stmt.query_map(&[], |row| (row.get(0), row.get(1)))
        .expect("Could not retrieve query_map results");

    note_iter.collect()
}

fn main() {
    let path_buffer = match env::home_dir() {
        Some(path) => path,
        None => {
            println!("Couldn't find home directory.");
            ::std::process::exit(1);
        }
    };

    let home_dir_path = path_buffer.to_str().unwrap();
    let mut config_file_path = format!("{}/slacknotify.json", home_dir_path);

    if !Path::new(&config_file_path).exists() {
        config_file_path = format!("{}/github_repos/rust/slacknotify/src/slacknotify.json",
                                   home_dir_path);
    }

    let file = File::open(config_file_path).expect("Json file not found.");

    let file_reader = BufReader::new(file);
    let config_json: Value = serde_json::from_reader(file_reader)
        .expect("Couldn't read json file.");

    let config_json = config_json
        .as_object()
        .expect("couldn't parse json file.  check syntax");

    let tmpdir = env::var("TMPDIR").expect("could not read TMPDIR env variable");
    let notificationcenter_path = format!("{}../0/com.apple.notificationcenter/db/db", tmpdir);
    let conn = Connection::open(notificationcenter_path).expect("could not open database");

    let mut newest_note = get_newest_note(&conn);

    loop {
        let note_iter = perform_db_lookup(newest_note, &conn);

        for note in note_iter {
            match note {
                Ok(note_data) => {
                    let (note_id, encoded_data): (u32, Option<Vec<u8>>) = note_data;
                    let encoded_data = encoded_data.unwrap();
                    let encoded_data = String::from_utf8_lossy(&encoded_data);

                    for team in config_json.iter() {
                        let (_, team_details) = team;
                        let members = team_details["members"].clone();
                        let sound = team_details["sound"].clone();

                        if members
                               .as_array()
                               .expect("team members aren't an array")
                               .iter()
                               .any(|data| encoded_data.contains(data.as_str().unwrap())) {
                            Command::new("sh")
                                .arg("-c")
                                .arg(&format!("afplay {}", sound))
                                .output()
                                .expect("afplay failed??");
                        }
                    }

                    newest_note = note_id;
                }
                Err(_) => continue,
            };
        }

        thread::sleep(time::Duration::from_secs(1));
    }
}
