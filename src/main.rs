extern crate actix_web;
#[macro_use] extern crate mysql;
#[macro_use] extern crate serde_derive;
extern crate futures;
extern crate rand;

use actix_web::{
    server,
    App,
    http,
    error,
    middleware::{
        Middleware,
        Started,
    },
    HttpRequest,
};
use std::sync::Arc;
mod url_controller;
mod user_controller;

#[derive(Debug)]
pub struct ApplicationState {
    pool: mysql::Pool,
}

struct PostMiddleware;
impl<S> Middleware<S> for PostMiddleware {
    fn start(&self, req: &mut HttpRequest<S>) -> error::Result<Started> {
        match req.method() {
            &http::Method::POST => Ok(Started::Done),
            _ => Ok(Started::Done),
        }
    }
}

fn main() {
    let pool = mysql::Pool::new("mysql://root:root@172.17.0.2:3306/short_url").unwrap();
    let state = Arc::new(ApplicationState {
        pool: pool
    });

    server::new(move || {
        App::with_state(state.clone())
            .middleware(PostMiddleware)
            .route("/register", http::Method::POST, url_controller::register)
            .route("/{id}", http::Method::GET, url_controller::get_url)
    }).bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
