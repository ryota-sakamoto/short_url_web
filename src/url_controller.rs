use actix_web::{
    http::StatusCode,
    HttpRequest,
    HttpResponse,
    Responder,
    Path,
};
use mysql::from_row;
use std::sync::Arc;
use ApplicationState;

pub fn get_url(req: HttpRequest<Arc<ApplicationState>>) -> impl Responder {
    let path = req.path();
    let id = path.replacen("/", "", 1);

    let state = req.state();
    let url_opt: Option<String> = state.pool.first_exec(r"
        select url from url_list where id = :id
    ", params!{
        "id" => id,
    }).unwrap().map(|r| {
        let url = from_row(r);
        url
    });

    match url_opt {
        Some(u) => {
            HttpResponse::Ok()
                .status(StatusCode::MOVED_PERMANENTLY)
                .header("Location", u)
                .finish()
        },
        None => {
            HttpResponse::Ok()
                .status(StatusCode::NOT_FOUND)
                .finish()
        },
    }
}

pub fn register_url(_: HttpRequest<Arc<ApplicationState>>) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}
pub fn remove_url(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}