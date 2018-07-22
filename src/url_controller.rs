use actix_web::{
    http::StatusCode,
    HttpRequest,
    HttpResponse,
    Responder,
    Path,
};
use std::sync::Arc;
use ApplicationState;

pub fn get_url(p: Path<String>) -> impl Responder {
    let id = p.into_inner();
    HttpResponse::Ok()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header("Location", "http://127.0.0.1:8080")
        .finish()
}

pub fn register_url(_: HttpRequest<Arc<ApplicationState>>) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}
pub fn remove_url(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}