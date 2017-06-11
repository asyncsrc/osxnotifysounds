extern crate serde_json;

use notificationcenter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_id_missing() {
        let expected_error = "app_id not found for application name: app_name";

        let json = r#"{
                        "applications": [{
                            "app_name": {
                                "bad_app_id": 99999,
                                "notification_details": {
                                    "group": {
                                        "sound": "/System/Library/Sounds/Blow.aiff",
                                        "look_for": [
                                            "text"
                                        ]
                                    }
                                }
                            }
                        }]
                      }"#;

        let config_json =
            serde_json::from_str(json).expect("could not convert string to json value");
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