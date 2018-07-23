use actix_web::{
    http::StatusCode,
    error,
    Error,
    HttpRequest,
    HttpMessage,
    HttpResponse,
    AsyncResponder,
    Responder,
    Path,
};
use futures::future::Future;
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
    }).unwrap().map(::mysql::from_row);

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

#[derive(Debug, Serialize, Deserialize)]
struct RegisterRequest {
    url: String,
}

pub fn register(req: HttpRequest<Arc<ApplicationState>>) -> Box<Future<Item=impl Responder, Error=Error>> {
    req.json()
        .from_err()
        .and_then(validate_url)
        .and_then(register_url)
        .responder()
}

fn register_url(req: RegisterRequest) -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().finish())
}

fn validate_url(req: RegisterRequest) -> Result<RegisterRequest, Error> {
    if req.url.starts_with("http://") || req.url.starts_with("https://") {
        Ok(req)
    } else {
        Err(error::ErrorBadRequest(""))
    }
}

pub fn remove_url(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}