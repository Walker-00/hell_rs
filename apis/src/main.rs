/*  We'll use Actix as our web framework.
    You can use whatever you want like
    (Actix, Axum, Rocket) but Actix is My fav.
*/

use std::io::Result;

//  Importing The crates
use actix_web::{
    get,          /* route handler */
    web,          /* Essentials helper functions and types for application registration */
    App,          /* The top-level builder for an Actix Web application */
    HttpResponse, /* An outgoing response */
    HttpServer,   /* An Http Server */
    Responder,    /* Trait implemented by types that can be converted to an HTTP response */
};

//  Main should be async so we can use async/await feature
//  So we'll use tokio main for making main func to async func

#[tokio::main] // this is call as macro, this will help to use async/await
async fn main() -> Result<()> {
    // First We gonna make http web server with actix
    /* HttpServer::new() will make new http server
     *  that HttpServer::new() take a closure And return Self
     *  App::new() crate the application to handle the requests known as api
     *  service() take HttpServiceFactory like HttpResponse and many others
     *  that services handle the requests and do something and return
     *  bind() take address(host and post) to bind the server
     *  in this tuto I'll use 127.0.0.1(localhost) and port 8090
     *  run() is a func that start listening for incoming connections
     *  The await keyword suspends the execution of an asynchronous function until the awaited future */
    HttpServer::new(|| App::new().service(ext))
        .bind(("127.0.0.1", 8090))?
        .run()
        .await
}

/*  Now We need to make extractor func,
    Which will extract data from request url
    and do something with that and response something
*/

/*  For making extractor we need to use macro,
    (get, post, put, delete) for this I will show you with get
    Try yourself for others methods
*/
#[get("/user/{name}")]
/* that macro will extract /user/{name} as func ext's name param as web path string and
*  func ext will return Responder Trait type */
/* func ext is also async func and that take name param as web path string type so get macro will
*  know to extract /user/{name} as String type */
/* For example if I request like /user/walker get macro will extract as walker cuz
*  walker is in {name} place so func ext's name param will get walker as input
*  so that will return like Hello walker!*/
async fn ext(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
