extern crate rusqlite;
extern crate serde_json;

use super::std::io::{Write,stderr};
use super::std::result::Result;

fn get_last_note_for_app(app_id: u32, conn: &rusqlite::Connection) -> u32 {
    let query = format!("SELECT note_id from notifications where app_id = {}
         order by note_id desc limit 1",
                        app_id);

    match conn.query_row(&query, &[], |row| row.get(0)) {
        Ok(entry) =>  entry,
        Err(err) => {
            writeln!(&mut stderr(),
                     "Error occurred: {}.  Waiting for first event for app: {}",
                     err,
                     app_id)
                    .unwrap();
            0
        }
    }
}

pub fn populate_app_notes(config_json: serde_json::Value, conn: &rusqlite::Connection)
    -> Vec<(u32, serde_json::Value)> {

    let mut app_notes: Vec<(u32, serde_json::Value)> = Vec::new();

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
                    &conn
            );
            app_notes.push((newest_note, app_details.clone()));
        }
    }

    app_notes
}

pub fn get_newest_alerts_for_app<X, Y>(newest_note: u32,
                               app_id: u32,
                               conn: &rusqlite::Connection)
                               -> Vec<Result<(X, Y), rusqlite::Error>>
    where X: rusqlite::types::FromSql,
          Y: rusqlite::types::FromSql
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

    let note_iter = stmt.query_map(&[], |row| (row.get(0), row.get(1)))
        .expect("Could not retrieve query_map results");

    note_iter.collect()
}
