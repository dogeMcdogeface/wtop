use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use actix_web::web::Data;
use rand::Rng;
use serde::{Deserialize, Serialize};

//------------------------------------------------------------------------------------------------//
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub poll_rate: u64,
}

//------------------------------------------------------------------------------------------------//
#[derive(Debug, Serialize, Default)]
pub struct SystemStatus {
    pub governor: u64,
    pub temp1: u64,
    pub freq1: u64,
    pub gpu_temp: String,
    pub zzz: u64,
}


//------------------------------------------------------------------------------------------------//
pub fn start(config: Config) -> Data<Mutex<SystemStatus>> {
    let data = Data::new(Mutex::new(SystemStatus::default()));
    let data_clone = Data::clone(&data);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(config.poll_rate));
            *data_clone.lock().unwrap() = read_stats();
        }
    });
    return data;
}


fn read_stats() -> SystemStatus {
    let mut rng = rand::thread_rng();
    let system_status = SystemStatus {
        governor: rng.gen(),
        temp1: rng.gen(),
        freq1: rng.gen(),
        gpu_temp: "Sample GPU Temp".to_owned(),
        zzz: rng.gen(),
    };
    return system_status;
}
