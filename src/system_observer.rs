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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_stats() {
        let stats = read_stats();
        assert_ne!(stats.governor, 0); // Test governor value
        assert_ne!(stats.temp1, 0); // Test temp1 value
        assert_ne!(stats.freq1, 0); // Test freq1 value
        assert_ne!(stats.gpu_temp, ""); // Test gpu_temp value
        assert_ne!(stats.zzz, 0); // Test zzz value
    }

    #[actix_rt::test]
    async fn test_start() {
        let test_poll_rate = 1;
        let config = Config { poll_rate: test_poll_rate };
        let data = start(config);

        // Wait for a couple of poll cycles
        thread::sleep(Duration::from_secs(test_poll_rate*2));

        let locked_data = data.lock().unwrap();

        // Access the fields of SystemStatus within the locked scope
        let governor = locked_data.governor;
        let temp1 = locked_data.temp1;
        let freq1 = locked_data.freq1;
        let gpu_temp = locked_data.gpu_temp.clone();
        let zzz = locked_data.zzz;

        drop(locked_data); // Release the lock

        // Check that the SystemStatus fields have been updated
        assert_ne!(governor, 0);
        assert_ne!(temp1, 0);
        assert_ne!(freq1, 0);
        assert_ne!(gpu_temp, "");
        assert_ne!(zzz, 0);
    }
}