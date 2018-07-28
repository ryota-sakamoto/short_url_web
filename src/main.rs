extern crate actix_web;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;
extern crate crypto;
extern crate futures;
extern crate rand;

use actix_web::{
    error,
    http,
    middleware::{Middleware, Started},
    server,
    App,
    HttpRequest,
};
use std::{env, sync::Arc};
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
    let db_ip = env::var_os("SHORT_URL_DB_IP")
        .expect("SET SHORT_URL_DB_IP")
        .into_string()
        .unwrap();

    let pool = mysql::Pool::new(format!("mysql://root:root@{}:3306/short_url", db_ip))
        .expect("MySQL Pool Error");
    let state = Arc::new(ApplicationState { pool: pool });

    server::new(move || {
        App::with_state(state.clone())
            .middleware(PostMiddleware)
            .route("/register", http::Method::POST, url_controller::register)
            .route("/{id}", http::Method::GET, url_controller::get_url)
    }).bind("127.0.0.1:8080")
        .expect("Server init error")
        .run();
}
