use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub poll_rate: i32,
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

pub fn init(app_name: &str, config_name: Option<&str>) -> Config {
    let config_location = confy::get_configuration_file_path(app_name, config_name)
        .expect("Failed to retrieve configuration directory");
    println!("Config File Location: {}", config_location.display());
    let config = confy::load::<Config>(app_name, config_name).unwrap_or_default();
    println!("Loaded Setting: {:?}", config);
    return config;
}

