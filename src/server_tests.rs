use std::fs;

#[cfg(test)]

use actix_web::{App, test};

use crate::server::serve_api;
use crate::server::serve_index;

#[actix_rt::test]
async fn test_index() {
    // Create an Actix web application instance with the `index()` service
    let mut app = test::init_service(App::new().service(serve_index)).await;

    // Send a GET request to the root path "/"
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response status is OK and the body contains the expected contents
    assert!(resp.status().is_success());
    let expected_content = fs::read_to_string("./www/index.html").unwrap();
    let body = test::read_body(resp).await;
    assert_eq!(body, expected_content);
}

#[actix_rt::test]
async fn test_api() {
    // Create an Actix web application instance with the `api()` service
    let mut app = test::init_service(App::new().service(serve_api)).await;

    // Send a GET request to the "/api/test" path
    let req = test::TestRequest::get().uri("/api/test").to_request();
    let resp = test::call_service(&mut app, req).await;

    // Assert that the response status is OK and the body contains the expected JSON content
    assert!(resp.status().is_success());
    let expected_json = r#"{"content":"test"}"#;
    let body = test::read_body(resp).await;
    assert_eq!(body, expected_json);
}
