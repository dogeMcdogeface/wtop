use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use actix_web::web::Data;
use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::system_status::SystemStatus;
use crate::system_status::SystemStatusValue;

//------------------------------------------------------------------------------------------------//
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub poll_rate: u64,
    pub permissions: SystemStatus,
}

//------------------------------------------------------------------------------------------------//
pub fn start(config: Config) -> Data<Mutex<SystemStatus>> {
    let data = Data::new(Mutex::new(SystemStatus::default()));
    let data_clone = Data::clone(&data);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(config.poll_rate));
            *data_clone.lock().unwrap() = read_stats(&config);
        }
    });
    return data;
}


fn read_stats(config: &Config) -> SystemStatus {
    let system_status = SystemStatus {
        read_at: Utc::now(),
        governor: SystemStatusValue::new_if_enabled(read_governor, &config.permissions.governor),
        temp1: SystemStatusValue::new_if_enabled(read_temp1, &config.permissions.temp1),
        freq1: SystemStatusValue::new_if_enabled(read_freq1, &config.permissions.freq1),
        gpu_temp: SystemStatusValue::new_if_enabled(read_gpu_temp, &config.permissions.gpu_temp),
        zzz: SystemStatusValue::new_if_enabled(read_zzz, &config.permissions.zzz),
    };
    return system_status;
}

//------------------------------------------------------------------------------------------------//

fn read_governor() -> u64 {
    // Example implementation for governor field
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn read_temp1() -> u64 {
    // Example implementation for temp1 field
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn read_freq1() -> u64 {
    // Example implementation for freq1 field
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn read_gpu_temp() -> u64 {
    // Example implementation for gpu_temp field
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn read_zzz() -> u64 {
    // Example implementation for zzz field
    let mut rng = rand::thread_rng();
    rng.gen()
}


//------------------------------------------------------------------------------------------------//
//                                          TESTS                                                 //
//------------------------------------------------------------------------------------------------//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_stats() {
        let stats = read_stats(&Default::default());
        assert_ne!(stats.governor, Default::default()); // Test governor value
        assert_ne!(stats.temp1, Default::default()); // Test temp1 value
        assert_ne!(stats.freq1, Default::default()); // Test freq1 value
        assert_ne!(stats.gpu_temp, Default::default()); // Test gpu_temp value
        assert_ne!(stats.zzz, Default::default()); // Test zzz value
    }

    #[actix_rt::test]
    async fn test_start() {
        let test_poll_rate = 1;
        let config = Config { poll_rate: test_poll_rate, permissions: Default::default() };
        let data = start(config);

        // Wait for a couple of poll cycles
        thread::sleep(Duration::from_secs(test_poll_rate*2));

        let default_status = SystemStatus::default();
        let locked_status = data.lock().unwrap();

        // Check that the SystemStatus fields have been updated
        assert_ne!(locked_status.governor, default_status.governor);
        assert_ne!(locked_status.temp1, default_status.temp1);
        assert_ne!(locked_status.freq1, default_status.freq1);
        assert_ne!(locked_status.gpu_temp, default_status.gpu_temp);
        assert_ne!(locked_status.zzz, default_status.zzz);

        drop(locked_status); // Release the lock
    }
}