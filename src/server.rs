use serde::{Deserialize, Serialize};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::fs;


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: String,
}


#[derive(Serialize)]
struct ApiResponse {
    content: String,
}

#[get("/")]
pub(crate) async fn serve_index() -> impl Responder {
    // Read the contents of the index.html file
    let content = fs::read_to_string("./www/index.html").unwrap();
    HttpResponse::Ok().body(content)
}

#[get("/api/{path:.*}")]
pub(crate) async fn serve_api(path: web::Path<String>) -> impl Responder {
    let content = path.into_inner();
    // Construct a JSON response with the contents of the request after "/api/"
    let response = ApiResponse { content };
    HttpResponse::Ok().json(response)
}


pub async fn run(config: Config) -> std::io::Result<()> {
    let server_address = format!("{}:{}", config.host, config.port);


    println!("Starting server at http://{}", server_address);
    HttpServer::new(|| App::new()
        .service(serve_index)
        .service(serve_api)
    )
        .bind(server_address)?
        .run()
        .await
}



