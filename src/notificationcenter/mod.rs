extern crate rusqlite;

use std::io::Write;
use super::std;

pub fn get_newest_note(app_id: u32, conn: &rusqlite::Connection) -> u32 {
    let query = format!("SELECT note_id from notifications where app_id = {}
         order by note_id desc limit 1",
                        app_id);

    match conn.query_row(&query, &[], |row| row.get(0)) {
        Ok(entry) =>  entry,
        Err(err) => {
            writeln!(&mut std::io::stderr(),
                     "Error occurred: {}.  Waiting for first event for app: {}",
                     err,
                     app_id)
                    .unwrap();
            0
        }
    }
}

pub fn perform_db_lookup<X, Y>(newest_note: u32,
                               app_id: u32,
                               conn: &rusqlite::Connection)
                               -> Vec<std::result::Result<(X, Y), rusqlite::Error>>
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
