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
    pub app_id: u32,
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
pub fn open_notificationcenter_db() -> Result<rusqlite::Connection, String> {
     env::var("TMPDIR")
     .map_err(|err| err.to_string())
     .and_then(|path| {
        let nc_path = format!("{}../0/com.apple.notificationcenter/db/db", path);
        Connection::open(nc_path).map_err(|err| err.to_string())
    })
}

pub fn populate_app_notes(config_json: &serde_json::Value, conn: &rusqlite::Connection)
    -> Result<Vec<AppNotes>, String> {

    let mut app_notes: Vec<AppNotes> = Vec::new();

    let app_iter = config_json
        .get("applications")
        .and_then(|app| app.as_array())
        .ok_or("applications section of config does not appear to be an array\
        , or it does not exist at all.")?;

    for app in app_iter {
        if let Some(obj) = app.as_object() {
            for (name, details) in obj {
                print!("Gathering details for app: {}. ", name);
                details.get("app_id")
                .and_then(|id| id.as_u64())
                .ok_or_else(|| "Could not map app_id inside config to positive integer".to_string())
                .map(|id| {
                    println!("Found app_id: {}", id);
                    app_notes.push(
                        AppNotes {
                            app_id: id as u32,
                            note_id: get_last_note_for_app(id as u32, conn),
                            details: details.clone()
                        }
                    )
                })?;
            }
        }
    }
    println!("Detail gathering complete.");
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