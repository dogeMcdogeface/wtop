mod server;
mod system_observer;
mod settings_loader;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = settings_loader::init("my_app_name", None);
    println!("Loaded Setting: {:?}", config);

    let server_address = format!("{}:{}", config.host, config.port);
    server::run(&server_address).await
}

#[cfg(test)]
mod server_tests {
    // your server tests
}
