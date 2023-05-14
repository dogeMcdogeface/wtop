use std::thread;
use std::time::Duration;

pub fn start() {
    thread::spawn(|| {
        loop {
            println!("hello");
            thread::sleep(Duration::from_secs(20));
        }
    });
}
