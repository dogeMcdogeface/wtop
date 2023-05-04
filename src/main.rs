mod server;

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = env::var("PORT").unwrap_or_else(|_| String::from("3680"));

    let server_address = format!("{}:{}", host, port);

    server::run(&server_address).await
}
