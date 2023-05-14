mod server;
mod system_observer;
mod settings_loader;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server : server::Config,
    pub observer : system_observer::Config,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: server::Config {
                host: String::from("127.0.0.1"),
                port: String::from("3680"),},
            observer: system_observer::Config { poll_rate: 5 },
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //Load the config file
    let config : Config = settings_loader::init("config.toml");

    // Start the system observer in a separate thread
    system_observer::start(config.observer);

    // Start the server
    server::run(config.server).await
}



#[cfg(test)]
mod server_tests {
    // your server tests
}
