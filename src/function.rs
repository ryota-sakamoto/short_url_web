use actix_web::{
    http::StatusCode,
    HttpRequest,
    HttpResponse,
    Responder,
    Path,
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

pub fn get_url(p: Path<String>) -> impl Responder {
    let id = p.into_inner();
    HttpResponse::Ok()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header("Location", "http://127.0.0.1:8080")
        .finish()
}

pub fn add_short_url(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}
pub fn remove_short_url(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}