use actix_files::NamedFile;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
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
    Ok(NamedFile::open("./src/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api)
            .route("/index", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
