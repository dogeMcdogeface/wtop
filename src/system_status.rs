use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

//----------------------------- SYSTEM STATUS STRUCT ---------------------------------------------//
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemStatus {
    #[serde( skip_deserializing, skip_serializing_if = "should_skip_read_at" )]
    pub read_at: DateTime<Utc>,
    pub governor: SystemStatusValue,
    pub temp1: SystemStatusValue,
    pub freq1: SystemStatusValue,
    pub gpu_temp: SystemStatusValue,
    pub zzz: SystemStatusValue,
}

fn should_skip_read_at(read_at: &DateTime<Utc>) -> bool {
    read_at.timestamp() == 0
}

impl SystemStatus {
    pub fn new_with_auth(source: &SystemStatus) -> SystemStatus {
        SystemStatus {
            read_at: source.read_at,
            governor:  SystemStatusValue::new_with_auth(&source.governor),
            temp1:  SystemStatusValue::new_with_auth(&source.temp1),
            freq1:  SystemStatusValue::new_with_auth(&source.freq1),
            gpu_temp:  SystemStatusValue::new_with_auth(&source.gpu_temp),
            zzz:  SystemStatusValue::new_with_auth(&source.zzz),
        }
    }
}


//----------------------------- SYSTEM STATUS VALUE STRUCT ---------------------------------------//
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemStatusValue {
    //#[config(skip)]
    value: u64,
    pub lock: StatusEnum,
}

impl Default for SystemStatusValue {
    fn default() -> Self {
        Self{ value: 0, lock: StatusEnum::Auth }
    }
}

impl SystemStatusValue {
    pub fn new_if_enabled<F>(read_fn: F, source: &SystemStatusValue) -> SystemStatusValue
        where F: FnOnce() -> u64,  {
        let lock = source.lock.clone();
        Self {
            value: if lock == StatusEnum::Disable { 0 } else { read_fn() },
            lock,
        }
    }

    fn new_with_auth(source: &SystemStatusValue) -> SystemStatusValue {
        let lock = source.lock.clone();
        SystemStatusValue {
            value: if lock == StatusEnum::Auth { 0 } else { source.value },
            lock,
        }
    }
}

//----------------------------- SYSTEM STATUS VALUE ENUM -----------------------------------------//
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum StatusEnum {
    Disable,
    Public,
    Auth,
}


//------------------------------------------------------------------------------------------------//
//                                          TESTS                                                 //
//------------------------------------------------------------------------------------------------//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_status_new_with_auth() {
        let source = SystemStatus {
            read_at: Default::default(),
            governor: SystemStatusValue {
                value: 10,
                lock: StatusEnum::Auth,
            },
            temp1: SystemStatusValue {
                value: 20,
                lock: StatusEnum::Auth,
            },
            freq1: SystemStatusValue {
                value: 30,
                lock: StatusEnum::Auth,
            },
            gpu_temp: SystemStatusValue {
                value: 40,
                lock: StatusEnum::Auth,
            },
            zzz: SystemStatusValue {
                value: 50,
                lock: StatusEnum::Public,
            },
        };

        let expected = SystemStatus {
            read_at: Default::default(),
            governor: SystemStatusValue {
                value: 0,
                lock: StatusEnum::Auth,
            },
            temp1: SystemStatusValue {
                value: 0,
                lock: StatusEnum::Auth,
            },
            freq1: SystemStatusValue {
                value: 0,
                lock: StatusEnum::Auth,
            },
            gpu_temp: SystemStatusValue {
                value: 0,
                lock: StatusEnum::Auth,
            },
            zzz: SystemStatusValue {
                value: 50,
                lock: StatusEnum::Public,
            },
        };

        let result = SystemStatus::new_with_auth(&source);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_status_value_new_if_enabled_disable() {        // Test case 1: StatusEnum::Disable
        let source = SystemStatusValue {
            value: 0,
            lock: StatusEnum::Disable,
        };
        let result = SystemStatusValue::new_if_enabled(|| 42, &source);
        assert_eq!(result, source);
    }
    #[test]
    fn test_status_value_new_if_enabled_auth() {         // Test case 2: StatusEnum::Auth
        let source = SystemStatusValue {
            value: 100,
            lock: StatusEnum::Auth,
        };
        let result = SystemStatusValue::new_if_enabled(|| 42, &source);
        assert_eq!(result.value, 42);
        assert_eq!(result.lock, StatusEnum::Auth);
    }
    #[test]
    fn test_status_value_new_if_enabled_public() {        // Test case 3: StatusEnum::Public
        let source = SystemStatusValue {
            value: 100,
            lock: StatusEnum::Public,
        };
        let result = SystemStatusValue::new_if_enabled(|| 42, &source);
        assert_eq!(result.value, 42);
        assert_eq!(result.lock, StatusEnum::Public);
    }
}