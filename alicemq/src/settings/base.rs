
use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            host: std::env::var("host").unwrap(),
            port: std::env::var("port").unwrap().parse().unwrap(),
            username: std::env::var("username").unwrap(),
            password: std::env::var("password").unwrap(),
        }
    }
}
