use std::fs;
use std::ops::Deref;
use std::sync::Mutex;

use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::web::Data;
use serde::{Deserialize, Serialize};

use crate::system_observer::SystemStatus;

//------------------------------------------------------------------------------------------------//
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: String,
}
#[derive(Serialize)]
struct ContentResponse<'a> {
    content: String,   // Assuming ContentType is the type of `path.into_inner()`
    response: &'a SystemStatus, // Assuming SystemStatus is the type of `system_status.deref()`
}

//------------------------------------------------------------------------------------------------//
#[get("/")]
async fn serve_index() -> impl Responder {
    // Read the contents of the index.html file
    let content = fs::read_to_string("./www/index.html").unwrap();
    HttpResponse::Ok().body(content)
}

#[get("/api/{path:.*}")]
async fn serve_api(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let status_mutex = req.app_data::<Data<Mutex<SystemStatus>>>().unwrap();
    let system_status = status_mutex.lock().unwrap();



    let content = path.into_inner();
    let response = system_status.deref();
    let content_response = ContentResponse {
        content,
        response,
    };
    HttpResponse::Ok().json(content_response)
}


//------------------------------------------------------------------------------------------------//
pub async fn run(config: Config, status_mutex: Data<Mutex<SystemStatus>>) -> std::io::Result<()> {
    let server_address = format!("{}:{}", config.host, config.port);
    println!("Starting server at http://{}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&status_mutex))
            .service(serve_index)
            .service(serve_api)
    })
        .bind(server_address)?
        .run()
        .await
}



