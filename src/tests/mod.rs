extern crate serde_json;

use notificationcenter;
use std::io::BufReader;
use std::fs::File;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_id_missing() {
        let expected_error = "app_id not found for application name: app_name";

        let file =
            File::open("src/tests/app_id_missing.json.test")
                .expect("JSON test file not found.");

        let file_reader = BufReader::new(file);
        let config_json = serde_json::from_reader(file_reader).unwrap();
        let conn = notificationcenter::open_notificationcenter_db();
        let notes = notificationcenter::populate_app_notes(&config_json, &conn);

        match notes {
            Ok(val) => panic!("Received: {:?} but expected error: {}", val, expected_error),
            Err(err) => {
                assert_eq!(err, expected_error);
            }
        }
    }

    #[test]
    fn app_id_invalid() {
        let expected_error = "App id: -24 for application: app_name must be a valid number \
                             (i.e., not a string) and greater than 0";

        let file =
            File::open("src/tests/app_id_invalid.json.test")
                .expect("JSON test file not found.");

        let file_reader = BufReader::new(file);
        let config_json = serde_json::from_reader(file_reader).unwrap();
        let conn = notificationcenter::open_notificationcenter_db();
        let notes = notificationcenter::populate_app_notes(&config_json, &conn);

        match notes {
            Ok(val) => panic!("Received: {:?} but expected error: {}", val, expected_error),
            Err(err) => {
                assert_eq!(err, expected_error);
            }
        }
    }
}