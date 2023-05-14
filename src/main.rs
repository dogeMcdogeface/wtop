use crate::server::run;
use std::env;
mod server;
mod settings_loader;
mod system_observer;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = env::var("PORT").unwrap_or_else(|_| String::from("3680"));

    let server_address = format!("{}:{}", host, port);

    run(&server_address).await
}


pub fn add(a: i32, b: i32) -> i32 {
    a + b
}



#[cfg(test)]
mod server_tests;

