use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub poll_rate: u64,
}



pub fn start(config: Config) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(config.poll_rate));
        }
    });
}
