use actix_files::NamedFile;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}

#[get("/api")]
async fn api() -> impl Responder {
    let message = Message {
        message: String::from("Hello, world!"),
    };
    HttpResponse::Ok().json(message)
}

async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./www/index.html")?)
}

pub async fn run(server_address: &str) -> std::io::Result<()> {
    println!("Starting server at http://{}", server_address);
    HttpServer::new(|| {
        App::new()
            .service(api)
            .route("/", web::get().to(index))
    })
        .bind(server_address)?
        .run()
        .await
}
