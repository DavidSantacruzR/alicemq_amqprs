use dotenv::dotenv;
use std::collections::HashMap;

struct ConnectionSettings {
    host: String,
    port: u16,
    username: String,
    password: String
}

impl ConnectionSettings {
    fn new(host:String, port: u16, username: String, password: String) -> ConnectionSettings {
        ConnectionSettings {
           host, port, username, password
        }
    }

    fn load_settings() -> HashMap<&'static str, String> {
        dotenv().ok();
        let mut connection_settings = HashMap::new();
        for key in ["HOST", "PORT", "USERNAME", "PASSWORD"] {
            connection_settings.insert(key, std::env::var("HOST").unwrap());
        };
        connection_settings
    }
}

#[cfg(test)]
mod tests {
    fn test_load_env_settings() {}

    fn test_load_settings_default() {}
}

