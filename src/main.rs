extern crate actix_web;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;
extern crate crypto;
extern crate futures;
extern crate rand;

use actix_web::{
    http,
    middleware::{Finished, Middleware},
    server,
    App,
    HttpRequest,
    HttpResponse,
};
use std::{env, ffi::OsString, sync::Arc};
mod controller;
mod model;

#[derive(Debug)]
pub struct ApplicationState {
    hostname: String,
    pool: mysql::Pool,
}

struct ErrorMiddleware;
impl<S> Middleware<S> for ErrorMiddleware {
    fn finish(&self, _: &mut HttpRequest<S>, res: &HttpResponse) -> Finished {
        if let Some(error) = res.error() {
            println!("[ERROR]{}", error);
        }
        Finished::Done
    }
}

fn main() {
    let db_ip = env::var_os("SHORT_URL_DB_IP")
        .unwrap_or(OsString::from("localhost"))
        .into_string()
        .unwrap();

    let hostname = env::var_os("HOSTNAME")
        .unwrap_or(OsString::from("localhost"))
        .into_string()
        .unwrap();

    let pool = mysql::Pool::new(format!("mysql://root:root@{}:3306/short_url", db_ip))
        .expect("MySQL Pool Error");
    let state = Arc::new(ApplicationState {
        pool: pool,
        hostname: hostname,
    });

    server::new(move || {
        App::with_state(state.clone())
            .middleware(ErrorMiddleware)
            .route(
                "/register",
                http::Method::POST,
                controller::url_controller::register,
            )
            .route(
                "/{id}",
                http::Method::GET,
                controller::url_controller::get_url,
            )
    }).bind("127.0.0.1:8080")
        .expect("Server init error")
        .run();
}
