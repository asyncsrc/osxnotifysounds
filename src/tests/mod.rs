extern crate serde_json;

use notificationcenter;
use std::io::BufReader;
use std::fs::File;
use std::fmt::Debug;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_id_invalid() {
        let expected_error = "Could not map app_id inside config to positive integer";
        let files = vec!["negative", "string", "missing"];

        for file_topic in files {
            let file_path = format!("src/tests/app_id_{}.json.test", file_topic);
            let file = File::open(&file_path)
                            .expect(&format!("couldn't open json test file: {}", &file_path));

            let notes = get_json_object(&file);
            validate_response(notes, expected_error);
        }
    }

    fn validate_response<T>(notes: Result<T,String>, expected_error: &str)
        where T: Debug {
        match notes {
            Ok(val) => panic!("Received: {:?} but expected error: {}", val, expected_error),
            Err(err) => {
                assert_eq!(err, expected_error);
            }
        }
    }

    fn get_json_object(file: &File) -> Result<Vec<notificationcenter::AppNotes>, String> {
        let file_reader = BufReader::new(file);
        let config_json = serde_json::from_reader(file_reader).unwrap();
        let conn = notificationcenter::open_notificationcenter_db().unwrap();
        notificationcenter::populate_app_notes(&config_json, &conn)
        .map_err(|err| err.to_string())
    }
}