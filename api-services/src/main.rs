use redis::{RedisResult, RedisError, Client};

extern crate redis;
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/movie")]
fn movie() -> String {
    let client = redis::Client::open("127.0.0.1:6379").unwrap();
    let mut con = client.get_connection().unwrap();
    let movie : String = redis::cmd("GET").arg("movie").query(&mut con).unwrap();
    format!("{}", movie)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/movie", routes![movie])
}

