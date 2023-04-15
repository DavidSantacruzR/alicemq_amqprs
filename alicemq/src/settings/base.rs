
use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub channel: String,
    pub exchange: String
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            host: std::env::var("HOST").unwrap(),
            port:  std::env::var("PORT").unwrap().parse().unwrap(),
            username: std::env::var("USERNAME").unwrap(),
            password: std::env::var("PASSWORD").unwrap(),
            channel: std::env::var("CHANNEL").unwrap(),
            exchange: std::env::var("EXCHANGE").unwrap()
        }
    }
}
