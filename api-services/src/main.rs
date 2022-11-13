#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate redis;
extern crate rocket_contrib;

use std;
use redis::RedisError;
use rocket::{ignite, response::Debug};
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/movie")]
fn movie() -> Result<Json<String>, Debug<RedisError>> {
    let redis_client = redis::Client::open("redis://127.0.0.1:6379")?;
    let mut con = redis_client.get_connection()?;
    let movie: String = redis::cmd("JSON.GET").arg("movie").query(&mut con)?; 
    println!("{}", movie);
    Ok(Json(movie))
}

fn main() {
    ignite()
        .mount("/", routes![index])
        .mount("/api", routes![movie])
        .launch();
}
