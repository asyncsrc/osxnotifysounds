extern crate serde_json;

use notificationcenter;
use std::io::BufReader;
use std::fs::File;
use std::fmt::Debug;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_id_missing() {
        let expected_error = "app_id not found for application name: app_name";

        let file = File::open("src/tests/app_id_missing.json.test")
                        .expect("JSON test file not found.");

        let notes = get_json_object(&file);
        validate_response(notes, expected_error);
    }

    #[test]
    fn app_id_negative() {
        let expected_error = "App id: -24 for application: app_name must be a valid number \
                             (i.e., not a string) and greater than 0";

        let file =
            File::open("src/tests/app_id_negative.json.test")
                .expect("JSON test file not found.");

        let notes = get_json_object(&file);
        validate_response(notes, expected_error);
    }

    #[test]
    fn app_id_string() {
        let expected_error = "App id: \"24\" for application: app_name must be a valid number \
                             (i.e., not a string) and greater than 0";

        let file =
            File::open("src/tests/app_id_string.json.test")
                .expect("JSON test file not found.");

        let notes = get_json_object(&file);
        validate_response(notes, expected_error);
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