use std::sync::Mutex;

use actix_files as fs;
use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::web::Data;
use serde::{Deserialize, Serialize};

use crate::system_status::SystemStatus;

//------------------------------- CONFIG STRUCT --------------------------------------------------//
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: String,
}


//------------------------------- CUSTOM HANDLERS ------------------------------------------------//
#[get("/api/{path:.*}")]
async fn serve_api(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let path_inner = path.into_inner();
    let authorized = path_inner == "authorized";

    let system_status = req.app_data::<Data<Mutex<SystemStatus>>>()
        .unwrap()
        .lock()
        .unwrap();
    let auth_status = SystemStatus::new_with_auth(&system_status);


    #[derive(Serialize)]
    struct ContentResponse<'a> {
        content: String,
        response: &'a SystemStatus,
    }

    HttpResponse::Ok().json(ContentResponse {
        content: path_inner,
        response: if authorized { &system_status } else { &auth_status },
    })
}

//------------------------------- SERVER INIT ----------------------------------------------------//
pub async fn run(config: Config, status_mutex: Data<Mutex<SystemStatus>>) -> std::io::Result<()> {
    let server_address = format!("{}:{}", config.host, config.port);
    println!("Starting server at http://{}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&status_mutex))
            .service(serve_api)
            .service(fs::Files::new("/files", "./www").show_files_listing())//Order is important...
            .service(fs::Files::new("/", "./www").index_file("index.html"))
            //                   //TODO user authentication
            //.default_service() //TODO custom 404 handling
    })
        .bind(server_address)?
        .run()
        .await
}



//------------------------------------------------------------------------------------------------//
//                                          TESTS                                                 //
//------------------------------------------------------------------------------------------------//
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use std::fs::read_to_string;

    #[actix_rt::test]
    async fn test_serve_index() {
        // Create an Actix web application instance with the `index()` service
        let mut app = test::init_service(App::new().service(fs::Files::new("/", "./www").index_file("index.html"))).await;

        // Send a GET request to the root path "/"
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Assert that the response status is OK and the body contains the expected contents
        assert!(resp.status().is_success());
        let expected_content = read_to_string("./www/index.html").unwrap();
        let body = test::read_body(resp).await;
        assert_eq!(body, expected_content);
    }

    #[actix_rt::test]
    async fn test_serve_api() {
        let system_status = SystemStatus::default();
        let data = Data::new(Mutex::new(system_status));
        let mut app = test::init_service(App::new().app_data(data.clone()).service(serve_api)).await;

        let req = test::TestRequest::get().uri("/api/some/path").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let content_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(content_response["content"], "some/path");


        let default_status = SystemStatus::default();
        for (field, value) in content_response["response"].as_object().unwrap() {
            assert_eq!(value, &serde_json::to_value(&default_status).unwrap()[field]);
        }
    }
    #[test]
    async fn test_run() {
        let config = Config {
            host: "127.0.0.1".to_string(),
            port: "8080".to_string(),
        };
        let system_status = SystemStatus::default();
        let data = Data::new(Mutex::new(system_status));

        let handle = std::thread::spawn(move || {
            let runtime = actix_rt::Runtime::new().unwrap();
            let future = run(config, data);
            runtime.block_on(future).unwrap();
        });

        // Wait for a short period to allow the server to start
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Perform your assertions or make requests to the running server here

        // Optionally, you can gracefully stop the server by dropping the handle
        drop(handle);
    }

}
