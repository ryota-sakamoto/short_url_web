use actix_web::{
    http::StatusCode,
    HttpRequest,
    HttpResponse,
    Responder,
};

pub fn register(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)    
}

pub fn login(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}