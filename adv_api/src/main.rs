// Importing Libraries
use actix_cors::Cors; // For Cross-Origin Resource Sharing (CORS)
use actix_web::{
    web,          /* Essentials helper functions and types for application registration */
    App,          /* An outgoing response */
    HttpResponse, /* Trait implemented by types that can be converted to an HTTP response */
    HttpServer,   /* An Http Server */
};
use serde::{Deserialize, Serialize}; // Serializing and Deserializing json & data structures
use std::io::Result; // For Result return type

#[derive(Serialize, Deserialize)] // this derive macro tell struct Request can Serialize and Deserialize
struct Request {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[tokio::main] // tokio async/await macro
async fn main() -> Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive(); /* That will allow all permit to all origin don't use permissive for product config yourself with Cors::default() */
        App::new() // new() will crate new application
            .wrap(cors) // warp will register and app-wide middleware
            .app_data("Hello".to_string()) // application root level data
            .route("/user", web::post().to(handler)) // Configure the route for specific url path
    }) // Will crate new Http Server
    .bind(("127.0.0.1", 8090))? // Http Server will bind in 127.0.0.1(localhost) and port 8090
    .run()  // will start listening incoming requests
    .await
}

async fn handler(
    app_data: web::Data<String>,  /* will get app root level data */
    req_data: web::Json<Request>, /* will get Request type as post body */
) -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: format!("{:?} {}!", app_data, req_data.name),
    }) // this will response Response type as json with Ok (http code 200)
}

/*
    For more check out https://github.com/RizeKishimaro/Tracebook
*/
