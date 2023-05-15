use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn init<T>(config_name: &str) -> T
    where T: Debug + DeserializeOwned + Serialize + Default,
{
    let config_path = Path::new(config_name);
    let config: T;

    if !config_path.exists() {
        eprintln!("Configuration File does Not Exist. Creating with default values");
        config = Default::default();
        save_config(config_name, &config);
        println!("Created Configuration File at: {}", config_path.canonicalize().unwrap().display());
        println!("Loaded Setting: {:?}", config);
        return config;
    }

    println!("Loading Configuration File at: {}", config_path.canonicalize().unwrap().display());
    let file = File::open(&config_path);
    if let Err(ref err) = file {
        eprintln!("Failed to open config file: {}", err);
        config = Default::default();
        save_config(config_name, &config);
        println!("Falling back on default values, Loaded Setting: {:?}", config);
        return config;
    }

    let mut file = file.unwrap();
    let mut contents = String::new();
    if let Err(ref err) = file.read_to_string(&mut contents) {
        eprintln!("Failed to read config file: {}", err);
        config = Default::default();
        save_config(config_name, &config);
        println!("Falling back on default values, Loaded Setting: {:?}", config);
        return config;
    }
    let result = toml::from_str(&contents);
    if let Err(ref err) = result {
        eprintln!("Failed to parse config file: {}", err);
        config = Default::default();
        println!("Falling back on default values, Loaded Setting: {:?}", config);
        return config;
    }
    config = result.unwrap();
    println!("Loaded Setting: {:?}", config);
    return config;
}

fn save_config<T>(config_name: &str, config: &T)
    where T: Serialize,
{
    let config_location = Path::new(config_name);
    let toml_str = toml::to_string(config)
        .map_err(|err| format!("Failed to serialize config to TOML: {}", err));

    if let Err(ref err) = toml_str {
        eprintln!("{}", err);
        return;
    }

    let file = File::create(config_location)
        .map_err(|err| format!("Failed to create config file: {}", err));

    if let Err(ref err) = file {
        eprintln!("{}", err);
        return;
    }

    if let Err(err) = file.unwrap().write_all(toml_str.unwrap().as_bytes()) {
        eprintln!("Failed to write config file: {}", err);
    }
}
