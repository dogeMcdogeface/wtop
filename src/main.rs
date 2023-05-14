mod server;
mod system_observer;
mod settings_loader;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server : server::Config,
    pub poll_rate: i32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: server::Config {
                host: String::from("127.0.0.1"),
                port: String::from("3680"),},
            poll_rate: 10,
        }
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //Load the config file
   /* let config = settings_loader::init("wtop", None);
    println!("Loaded Settings: {:?}", config);*/
    let config : Config = settings_loader::init("config.toml");


    // Start the system observer in a separate thread
    system_observer::start();

    // Start the server
    let server_address = format!("{}:{}", config.server.host, config.server.port);
    server::run(&server_address).await
}
#[cfg(test)]
mod server_tests {
    // your server tests
}
