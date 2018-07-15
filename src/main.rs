extern crate actix_web;

use actix_web::{
    middleware::Middleware,
    server,
    App,
};

fn main() {
    server::new(|| {
        App::new()
    });
}
