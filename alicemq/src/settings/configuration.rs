use dotenv::dotenv;
use std::collections::HashMap;
use crate::settings::constants::{DEFAULT_HOST, DEFAULT_PASSWORD, DEFAULT_PORT, DEFAULT_USER};

#[derive(PartialEq, Debug)]
struct ConnectionSettings {
    host: String,
    port: u16,
    username: String,
    password: String
}

impl ConnectionSettings {

    #[allow(dead_code)]
    fn new() -> ConnectionSettings {
        let _settings = ConnectionSettings::load_settings();
        ConnectionSettings {
            host: String::from(_settings.get("HOST").unwrap()),
            port: _settings.get("PORT").unwrap().parse().expect("Unable to parse port number."),
            username: String::from(_settings.get("USERNAME").unwrap()),
            password: String::from(_settings.get("PASSWORD").unwrap())
        }
    }

    #[allow(dead_code)]
    fn load_settings() -> HashMap<&'static str, String> {
        dotenv().ok();
        let mut connection_settings = HashMap::new();
        connection_settings.insert("HOST", match std::env::var("HOST") {
            Ok(value) => { value }
            Err(_) => { DEFAULT_HOST.to_string() }
        });
        connection_settings.insert("PORT", match std::env::var("PORT") {
            Ok(value) => { value },
            Err(_) => { DEFAULT_PORT.to_string() }
        });
        connection_settings.insert("USERNAME", match std::env::var("USERNAME") {
            Ok(value) => { value },
            Err(_) => { DEFAULT_USER.to_string() }
        });
        connection_settings.insert("PASSWORD", match std::env::var("PASSWORD") {
            Ok(value) => { value },
            Err(_) => { DEFAULT_PASSWORD.to_string() }
        });
        connection_settings
    }
}

#[cfg(test)]
mod tests {
    use crate::settings::configuration::ConnectionSettings;
    use std::fs::{File, remove_file};
    use std::io::Write;

    #[test]
    fn test_load_env_settings() {
        let mut file = File::create(".env").expect("Unable to create .env file.");
        writeln!(file, "host=localhost").expect("did not set localhost.");
        writeln!(file, "port=5672").expect("did not set port.");
        writeln!(file, "username=guest").expect("did not set username.");
        writeln!(file, "password=guest").expect("did not set password.");
        let _settings = ConnectionSettings::new();
        let _manual_settings_object = ConnectionSettings {
            host: "localhost".to_string(),
            port: 5672,
            username: "guest".to_string(),
            password: "guest".to_string(),
        };
        let _ = remove_file(".env").is_ok();
        assert_eq!(_settings, _manual_settings_object)
    }

    #[test]
    fn test_load_settings_default() {}
}

