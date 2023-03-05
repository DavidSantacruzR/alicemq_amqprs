
use amqprs::connection::{OpenConnectionArguments};
use dotenv::dotenv;

#[derive(Debug)]
pub struct ConnectionArguments {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String
}

impl ConnectionArguments {
    pub fn load_from_env() -> OpenConnectionArguments {
        dotenv().ok();
        let host: String = std::env::var("host").expect("No host set.");
        let port: u16 = std::env::var("port").expect("No port set.").parse::<u16>().unwrap();
        let username: String = std::env::var("username").expect("No user set.");
        let password: String = std::env::var("password").expect("No pwd set.");

        OpenConnectionArguments::new(
            &host,
            port,
            &username,
            &password
        )
    }
}