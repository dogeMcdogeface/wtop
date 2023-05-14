use crate::server::run;
use confy::ConfyError;
use serde::{Deserialize, Serialize};

mod server;
mod system_observer;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    host: String,
    port: String,
    poll_rate: i32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: String::from("127.0.0.1"),
            port: String::from("3680"),
            poll_rate: 10,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Result<Config, ConfyError> = confy::load("my_app_name", None);
    let config = config.unwrap_or_default();
    let config_location = confy::get_configuration_file_path("my_app_name", None)
        .expect("Failed to retrieve configuration directory");
    println!("Config File Location: {}", config_location.display());


    let server_address = format!("{}:{}", config.host, config.port);
    run(&server_address).await
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod server_tests {
    // your server tests
}
