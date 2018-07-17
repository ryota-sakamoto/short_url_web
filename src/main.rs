extern crate actix_web;

use actix_web::{
    server,
    App,
    http,
};
mod function;

fn main() {
    server::new(|| {
        App::new()
            .route("/", http::Method::GET, function::index)
            .route("/register", http::Method::POST, function::register)
            .route("/login", http::Method::POST, function::login)
            .route("/logout", http::Method::POST, function::logout)
            .route("/{id}", http::Method::GET, function::get_url)
    }).bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
