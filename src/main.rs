use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::time::Instant;
#[macro_use] extern crate rocket;


fn elapsed_time(before: std::time::Instant)-> String {
    let elapsed = format!("Millis: {} ms", before.elapsed().as_millis());
    elapsed
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize, Deserialize)]
struct Ping {
    text: String,
    duration: String,
}

#[get("/ping")]
fn ping() -> Json<Ping> {
    let before = Instant::now();

    Json(Ping {
        text: "Pong!".to_string(),
        duration: elapsed_time(before), 
    })
}

#[get("/hello/<name>/<age>")]
fn hello_param(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[derive(Serialize, Deserialize)]
struct User {
    active: bool,
    username: String,
    email: String,
    age: u64,
    duration: String,
}

#[get("/user/<name>/<age>")]
fn user(name: &str, age: u64) -> Json<User> {
    let before = Instant::now();

    Json(User {
        active: true,
        username: name.to_string(),
        email: format!("{}@email.com", name),
        age: age,
        duration: elapsed_time(before), 
    })
}

#[derive(Serialize, Deserialize)]
struct LoopResponse {
    duration: String,
}

#[get("/loops")]
fn loops() -> Json<LoopResponse> {
    let before = Instant::now();
    
    println!("Start looping...");
    
    let mut sum = 0;
    for n in 1..1000 {
        sum += n;
    }

    println!("Total sum {}", sum);
    
    Json(LoopResponse {
        duration: elapsed_time(before)
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        hello, hello_param, user, loops, ping
    ])
}