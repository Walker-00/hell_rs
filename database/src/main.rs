/*  For Database I'll use Surrealdb which is
*   MultiModel NoSql All in One Database
*   But If you wanna use sql Db
*   Postgres is recommend
*   If you use sqldb use diesel or sqlx for
*   Compile time query checking to type safe your sql query
*/

// Importing Crates
use actix_web::{
    web,          /* Essentials helper functions and types for application registration */
    App,          /* An outgoing response */
    HttpResponse, /* Trait implemented by types that can be converted to an HTTP response */
    HttpServer, Scope, /* An Http Server */
};
use serde::{Deserialize, Serialize}; // Serializing and Deserializing json & data structures
use std::{
    collections::BTreeMap, /* For BTreeMap Data structure */
    io::Result,            /* For Result return type */
};
use surrealdb::{sql::Value, Datastore, Session};

type DB = (Datastore, Session); // custom data type for database datastore and session

#[derive(Serialize, Deserialize, Clone, Debug)] // This derive macro tell struct UserInfo can Serialize, Deserialize, Clone and Debug
struct UserInfo {
    name: String,
    age: u32,
    pass: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Response {
    message: String,
}

#[tokio::main] // tokio main macro to make async func
async fn main() -> Result<()> {
    // HttpServer::new() will create new http server
    HttpServer::new(|| {
        App::new() // will create new application
            .service(scopes()) // get service from scopes func
    })
    .bind(("127.0.0.1", 8090))? // will bind server in ip 127.0.0.1(localhost) port 8090
    .run() // Will start listening Incoming Http Request
    .await
}

// scopes func will return url scope
fn scopes() -> Scope {
    web::scope("/user").route("{method}", web::post().to(branches)) // This will create /user scope and get /user/{method} to extract & handle
}

// branches func will handle method and call func to do
async fn branches(method: web::Path<String>, info: web::Json<UserInfo>) -> HttpResponse {
    let db = &(
        Datastore::new("file://blabla.db").await.unwrap(),
        Session::for_db("ns", "db"),
    ); // I'm just fucking boring to add two cuz I make them as one data type

    // check method type (url param)
    match method.as_str() {
        "new" => crate_user(info, db).await, // if method is equ to new call crate_user func
        _ => HttpResponse::NotFound().await.unwrap(), // if not match with all just response NotFound
    }
}

// crate_user func will process user creating process
async fn crate_user(info: web::Json<UserInfo>, (ds, sess): &DB) -> HttpResponse {
    let sql = "CREATE user CONTENT $data"; // For more surrealql(SQL Like Language) check out https://surrealdb.com/docs/surrealql
    let info = info.clone(); // Just clone Value of info I'm so fucking boring to add clone
    let data: BTreeMap<String, Value> = [
        ("username".into(), info.name.into()),
        ("age".into(), info.age.into()),
        ("password".into(), info.pass.into()),
    ]
    .into(); // Storing user data in data BTreeMap(String and surreal Value) data structure

    let var: BTreeMap<String, Value> = [("data".into(), data.into())].into(); /* add data value to var variable to use in execute process */
    let resul = ds.execute(sql, sess, Some(var), false).await; /* will execute sql query and store user's data */

    // check resul for Ok or Error
    match resul {
        Ok(_) => HttpResponse::Ok().json(Response {
            message: "Authed".to_owned(),
        }), // Authed json Response will Response when there is no Error
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: e.to_string(),
        }), // This Error will response When Storing Data in Database has error
    }
}

/*
 * For more check out https://github.com/RizeKishimaro/Tracebook
 */
