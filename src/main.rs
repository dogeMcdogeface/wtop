use serde::{Deserialize, Serialize};

mod server;
mod system_observer;
mod settings_loader;
mod system_status;

//------------------------------------------------------------------------------------------------//
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: server::Config,
    pub observer: system_observer::Config,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: server::Config {
                host: String::from("127.0.0.1"),    //0.0.0.0  if configured to use this special address, the application will listen to any IP address configured on the machine.
                port: String::from("3680"),
            },
            observer: system_observer::Config {
                poll_rate: 1,
                permissions: Default::default(),
            },
        }
    }
}

//------------------------------------------------------------------------------------------------//
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //Load the config file
    let config: Config = settings_loader::init("config.toml");

    // Start the system observer in a separate thread
    let status_mutex = system_observer::start(config.observer);

    // Start the server
    server::run(config.server, status_mutex).await
}


//------------------------------------------------------------------------------------------------//
#[cfg(test)]
mod tests {
    // tests
}
