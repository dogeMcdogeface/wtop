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


//------------------------------------------------------------------------------------------------//
#[get("/")]
async fn serve_index() -> impl Responder {
    // Read the contents of the index.html file
    let content = fs::read_to_string("./www/index.html").unwrap();
    HttpResponse::Ok().body(content)
}

#[get("/api/{path:.*}")]
async fn serve_api(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let system_status = req.app_data::<Data<Mutex<SystemStatus>>>().unwrap().lock().unwrap();

    #[derive(Serialize)]
    struct ContentResponse<'a> {
        content: String,
        response: &'a SystemStatus,
    }

    HttpResponse::Ok().json(ContentResponse {
        content: path.into_inner(),
        response: system_status.deref(),
    })
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



