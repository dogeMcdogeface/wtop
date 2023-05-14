use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use serde::de::DeserializeOwned;

pub fn init<T>(config_name: &str) -> T
    where T: Debug + DeserializeOwned + Serialize + Default,
{
    let config_path = Path::new(config_name);
    let config: T;


    if !config_path.exists(){
        eprintln!("Configuration File does Not Exist. Creating with default values");
        config = Default::default();
        save_config(config_name, &config);
        println!("Created Configuration File at: {}", config_path.canonicalize().unwrap().display());
        println!("Loaded Setting: {:?}", config);
        return config;
    }else {
        println!("Loading Configuration File at: {}", config_path.canonicalize().unwrap().display());
        let mut file = File::open(&config_path);
        if let Err(ref err) = file {
            eprintln!("Failed to open config file: {}", err);
            config = Default::default();
            save_config(config_name, &config);
            println!("Falling back on default values, Loaded Setting: {:?}", config);
            return config;
        }else {
            let mut file = file.unwrap();
            let mut contents = String::new();
            if let Err(ref err) = file.read_to_string(&mut contents) {
                eprintln!("Failed to read config file: {}", err);
                config = Default::default();
                save_config(config_name, &config);
                println!("Falling back on default values, Loaded Setting: {:?}", config);
                return config;
            } else {
                let result = toml::from_str(&contents);
                if let Err(ref err) = result {
                    eprintln!("Failed to parse config file: {}", err);
                    config = Default::default();
                    println!("Falling back on default values, Loaded Setting: {:?}", config);
                    return config;
                } else {
                    config = result.unwrap();
                    println!("Loaded Setting: {:?}", config);
                    return config;
                }
            }
        }
    }





}

fn save_config<T>(config_name: &str, config: &T)
    where T: Serialize,
{
    let config_location = Path::new(config_name);
    let toml_str = toml::to_string(config).expect("Failed to serialize config to TOML");

    let mut file = File::create(config_location).expect("Failed to create config file");
    file.write_all(toml_str.as_bytes())
        .expect("Failed to write config file");
}
