extern crate rusqlite;
extern crate serde_json;

use super::std::result::Result;

pub struct AppLookup {
    pub app_id: u32,
    pub bundleid: String
}

pub struct NotificationLookup {
    pub note_id: u32,
    pub encoded_data: Vec<u8>
}

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
    -> Vec<AppNotes> {

    let mut app_notes: Vec<AppNotes> = Vec::new();

    let app_iter = config_json
        .get("applications")
        .unwrap()
        .as_array()
        .expect("applications value is not an array")
        .iter();

    for app in app_iter {
        for (_, app_details) in app.as_object().unwrap().iter() {
            let newest_note =
                get_last_note_for_app(
                    app_details
                    .get("app_id")
                    .unwrap()
                    .as_u64()
                    .unwrap() as u32,
                    conn
            );
            app_notes.push(AppNotes {
                note_id: newest_note,
                details: app_details.clone()
            });
        }
    }

    app_notes
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