use actix_web::{
    http::StatusCode,
    HttpRequest,
    HttpResponse,
    Responder,
};

pub fn index(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}

pub fn register(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)    
}

pub fn login(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}

pub fn logout(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}

pub fn add_short_url(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}
pub fn remove_short_url(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}