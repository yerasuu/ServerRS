#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::Config;
use rocket::config::Environment;
use rocket::fairing::AdHoc;

#[get("/<name>")]
fn named(name: String) -> String {
    format!("Hello, {}!", name)
}
#[get("/")]
fn index() -> String {
    format!("Hello, Home Page!")
}
fn main() {
    let config = Config::build(Environment::Development)
        .address("127.0.0.1")
        .port(700)
        .workers(12)
        .unwrap();
    rocket::custom(config)
        .mount("/", routes![index,named])
        .launch();
}