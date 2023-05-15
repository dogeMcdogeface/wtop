use serde::{Deserialize, Serialize};

//------------------------------------------------------------------------------------------------//
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemStatus {
    pub governor: StatusValue,
    pub temp1: StatusValue,
    pub freq1: StatusValue,
    pub gpu_temp: StatusValue,
    pub zzz: StatusValue,
}

impl SystemStatus {
    pub fn new_with_auth(source: &SystemStatus) -> SystemStatus {
        SystemStatus {
            governor:  StatusValue::new_with_auth(&source.governor),
            temp1:  StatusValue::new_with_auth(&source.temp1),
            freq1:  StatusValue::new_with_auth(&source.freq1),
            gpu_temp:  StatusValue::new_with_auth(&source.gpu_temp),
            zzz:  StatusValue::new_with_auth(&source.zzz),
        }
    }
}


//------------------------------------------------------------------------------------------------//
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusValue {
    value: u64,
    pub lock: StatusEnum,
}

impl Default for StatusValue {
    fn default() -> Self {
        Self{ value: 0, lock: StatusEnum::Auth }
    }
}

impl StatusValue {
    pub fn new_if_enabled<F>(read_fn: F, source: &StatusValue) -> StatusValue
        where F: FnOnce() -> u64,  {
        let lock = source.lock.clone();
        Self {
            value: if lock == StatusEnum::Disable { 0 } else { read_fn() },
            lock,
        }
    }

    fn new_with_auth(source: &StatusValue) -> StatusValue {
        let lock = source.lock.clone();
        StatusValue {
            value: if lock == StatusEnum::Auth { 0 } else { source.value },
            lock,
        }
    }
}

//------------------------------------------------------------------------------------------------//
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum StatusEnum {
    Disable,
    Public,
    Auth,
}


//------------------------------------------------------------------------------------------------//
//                               TESTS                                                            //
//------------------------------------------------------------------------------------------------//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_status_new_with_auth() {
        let source = SystemStatus {
            governor: StatusValue {
                value: 10,
                lock: StatusEnum::Auth,
            },
            temp1: StatusValue {
                value: 20,
                lock: StatusEnum::Auth,
            },
            freq1: StatusValue {
                value: 30,
                lock: StatusEnum::Auth,
            },
            gpu_temp: StatusValue {
                value: 40,
                lock: StatusEnum::Auth,
            },
            zzz: StatusValue {
                value: 50,
                lock: StatusEnum::Public,
            },
        };

        let expected = SystemStatus {
            governor: StatusValue {
                value: 0,
                lock: StatusEnum::Auth,
            },
            temp1: StatusValue {
                value: 0,
                lock: StatusEnum::Auth,
            },
            freq1: StatusValue {
                value: 0,
                lock: StatusEnum::Auth,
            },
            gpu_temp: StatusValue {
                value: 0,
                lock: StatusEnum::Auth,
            },
            zzz: StatusValue {
                value: 50,
                lock: StatusEnum::Public,
            },
        };

        let result = SystemStatus::new_with_auth(&source);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_status_value_new_if_enabled_disable() {        // Test case 1: StatusEnum::Disable
        let source = StatusValue {
            value: 0,
            lock: StatusEnum::Disable,
        };
        let result = StatusValue::new_if_enabled(|| 42, &source);
        assert_eq!(result, source);
    }
    #[test]
    fn test_status_value_new_if_enabled_auth() {         // Test case 2: StatusEnum::Auth
        let source = StatusValue {
            value: 100,
            lock: StatusEnum::Auth,
        };
        let result = StatusValue::new_if_enabled(|| 42, &source);
        assert_eq!(result.value, 42);
        assert_eq!(result.lock, StatusEnum::Auth);
    }
    #[test]
    fn test_status_value_new_if_enabled_public() {        // Test case 3: StatusEnum::Public
        let source = StatusValue {
            value: 100,
            lock: StatusEnum::Public,
        };
        let result = StatusValue::new_if_enabled(|| 42, &source);
        assert_eq!(result.value, 42);
        assert_eq!(result.lock, StatusEnum::Public);
    }
}