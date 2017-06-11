extern crate rusqlite;
extern crate serde_json;

use std::result::Result;
use rusqlite::Connection;
use std::env;

pub struct AppLookup {
    pub app_id: u32,
    pub bundleid: String
}

pub struct NotificationLookup {
    pub note_id: u32,
    pub encoded_data: Vec<u8>
}

#[derive(Debug)]
pub struct AppNotes {
    pub note_id: u32,
    pub details: serde_json::Value
}

fn get_last_note_for_app(app_id: u32, conn: &rusqlite::Connection) -> u32 {
    let query =
        format!(
            "SELECT note_id from notifications where app_id = {}
            order by note_id desc limit 1",
            app_id
    );

    match conn.query_row(&query, &[], |row| row.get(0)) {
        Ok(entry) =>  entry,
        Err(_) => 0
    }
}

pub fn populate_app_notes(config_json: &serde_json::Value, conn: &rusqlite::Connection)
    -> Result<Vec<AppNotes>, String> {

    let mut app_notes: Vec<AppNotes> = Vec::new();

    let app_iter = config_json
        .get("applications")
        .unwrap()
        .as_array()
        .expect("applications value is not an array")
        .iter();

    for app in app_iter {
        if let Some(app) = app.as_object() {
            for (app_name, app_details) in app {
                match app_details.get("app_id") {
                    Some(app_id) => {
                        if app_id.is_u64() {
                            let app_id = app_id.as_u64().unwrap();
                            app_notes.push(
                                AppNotes {
                                    note_id: get_last_note_for_app(app_id as u32, conn),
                                    details: app_details.clone()
                                }
                            );
                        }
                        else {
                            return Err(
                                format!(
                                    "App id: {} for application: {} must be a valid number (i.e., not a string) \
                                    and greater than 0",
                                    app_id,
                                    app_name
                                )
                            );
                        }
                    },
                    None => return Err(
                        format!("app_id not found for application name: {}", app_name)
                    )
                }
            }
        }
    }
    Ok(app_notes)
}

pub fn get_newest_alerts_for_app (
    newest_note: u32,
    app_id: u32,
    conn: &rusqlite::Connection
    ) -> Vec<Result<(NotificationLookup), rusqlite::Error>>
    {
    let mut stmt =
        conn.prepare(
            &format!(
                "SELECT note_id, encoded_data from notifications
                where app_id = {} and note_id > {} order by note_id",
                app_id,
                newest_note
            )
        ).expect("Could not prepare SQL select statement.");

    let note_iter = stmt.query_map(&[], |row| {
            NotificationLookup {
                note_id: row.get(0),
                encoded_data: row.get(1)
            }
        }).expect("Could not retrieve query_map results");

    note_iter.collect()
}

pub fn open_notificationcenter_db() -> rusqlite::Connection {
    let tmpdir = env::var("TMPDIR").expect("could not read TMPDIR env variable");
    let notificationcenter_path = format!("{}../0/com.apple.notificationcenter/db/db", tmpdir);
    Connection::open(notificationcenter_path).expect("could not open database")
}

pub fn lookup_app_id (
    app_name: &str,
    conn: &rusqlite::Connection
    ) -> Vec<Result<AppLookup, rusqlite::Error>>
    {
    let mut stmt = conn.prepare(
        &format!(
            "SELECT app_id, bundleid from app_info where bundleid like '%{}%'",
            app_name
        )
    ).expect("Could not prepare SQL select statement.");

    let app_iter = stmt.query_map(&[], |row| {
        AppLookup {
            app_id: row.get(0),
            bundleid: row.get(1)
        }
    }).expect("Could not retrieve query_map results");

    app_iter.collect()
}