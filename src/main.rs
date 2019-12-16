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
    // middleware::{Finished, Middleware},
    HttpServer,
    App,
    HttpRequest,
    HttpResponse,
    web,
};
use std::{sync::Arc};
mod controller;
mod model;

#[derive(Debug)]
pub struct ApplicationState {
    hostname: String,
    pool: mysql::Pool,
}

// struct ErrorMiddleware;
// impl<S> Middleware<S> for ErrorMiddleware {
//     fn finish(&self, _: &mut HttpRequest<S>, res: &HttpResponse) -> Finished {
//         if let Some(error) = res.error() {
//             println!("[ERROR]{}", error);
//         }
//         Finished::Done
//     }
// }

fn main() {
    let db_ip = option_env!("SHORT_URL_DB_IP").unwrap_or("localhost");
    let hostname = option_env!("HOSTNAME").unwrap_or("localhost");
    let db_user_name = option_env!("SHORT_URL_DB_USER").unwrap_or("root");
    let db_user_password = option_env!("SHORT_URL_DB_PASSWORD").unwrap_or("");
    let db_name = option_env!("SHORT_URL_DB_NAME").unwrap_or("short_url");
    let db_port = option_env!("SHORT_URL_DB_PORT").unwrap_or("3306");

    let pool = mysql::Pool::new(format!("mysql://{}:{}@{}:{}/{}", db_user_name, db_user_password, db_ip, db_port, db_name))
        .expect("MySQL Pool Error");
    let state = Arc::new(ApplicationState {
        pool: pool,
        hostname: hostname.to_string(),
    });

    let app_host = hostname.to_string() + ":8080";
    println!("start server: {}", app_host);

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .route(
                "/register",
                web::post().to(controller::url_controller::register)
            )
            .route(
                "/{id}",
                web::get().to(controller::url_controller::get_url)
            )
    }).bind(app_host)
        .expect("Server init error")
        .run();
}
