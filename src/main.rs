extern crate actix_web;
#[macro_use]
extern crate mysql;

use actix_web::{
    server,
    App,
    http,
};
use std::sync::Arc;
mod url_controller;
mod user_controller;

#[derive(Debug)]
pub struct ApplicationState {
    pool: mysql::Pool,
}

fn main() {
    let pool = mysql::Pool::new("mysql://root:root@172.17.0.2:3306/short_url").unwrap();
    let state = Arc::new(ApplicationState {
        pool: pool
    });

    server::new(move || {
        App::with_state(state.clone())
            .route("/register", http::Method::POST, url_controller::register_url)
            .route("/{id}", http::Method::GET, url_controller::get_url)
    }).bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
