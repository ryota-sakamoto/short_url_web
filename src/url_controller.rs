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
use rand::{
    Rng,
    thread_rng,
};

const ID_LEN: usize = 8;

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
    let pool = {
        let r = req.clone();
        let state = r.state();
        state.pool.clone()
    };
    req.json()
        .from_err()
        .and_then(validate_url)
        .and_then(move |v| register_url(v, pool))
        .responder()
}

// TODO
fn register_url(req: RegisterRequest, pool: ::mysql::Pool) -> Result<impl Responder, Error> {
    pool.prep_exec(r"
        insert into url_list values(:id, :url)
    ", params!{
        "id" => generate_id(),
        "url" => req.url
    }).map_err(|_| {
        error::ErrorInternalServerError("")
    })?;
    Ok(HttpResponse::Ok().finish())
}

fn generate_id() -> String {
    let char_vec: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890".chars().collect();
    let mut rng = thread_rng();
    let mut id = String::new();
    for _ in 0..ID_LEN {
        let n = rng.gen_range(0, char_vec.len());
        id.push(char_vec[n]);
    }
    id
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