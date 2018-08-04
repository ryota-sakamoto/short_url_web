use super::super::model::url_list;
use super::util;
use actix_web::{
    error, http::StatusCode, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse, Path,
    Responder,
};
use futures::future::Future;
use std::sync::Arc;
use ApplicationState;

const ID_LEN: usize = 8;

pub fn get_url(req: HttpRequest<Arc<ApplicationState>>) -> Result<impl Responder, Error> {
    let path = req.path();
    let id = path.replacen("/", "", 1);
    let q = req.query();
    let new_password = q.get("password").map(|s| format!("{}{}", s, id));

    let state = req.state();
    let url_result = url_list::find(&state.pool, id, new_password);

    let url_opt = url_result.map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(match url_opt {
        Some(u) => HttpResponse::Ok()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("Location", u.url)
            .finish(),
        None => HttpResponse::Ok().status(StatusCode::NOT_FOUND).finish(),
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct RegisterRequest {
    url: String,
    password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegisterResponse {
    id: String,
}

pub fn register(
    req: HttpRequest<Arc<ApplicationState>>,
) -> Box<Future<Item = impl Responder, Error = Error>> {
    let (pool, hostname) = {
        let r = req.clone();
        let state = r.state();
        (state.pool.clone(), state.hostname.clone())
    };
    req.json()
        .from_err()
        .and_then(move |v| validate_url(v, hostname))
        .and_then(move |v| register_url(v, pool))
        .responder()
}

// TODO transaction
fn register_url(req: RegisterRequest, pool: ::mysql::Pool) -> Result<impl Responder, Error> {
    let id = util::generate_id(ID_LEN);
    let new_password = req.password.map(|s| format!("{}{}", s, id));
    let url = req.url;

    url_list::insert(&pool, id.clone(), new_password, url)
        .map(|_| HttpResponse::Ok().json(RegisterResponse { id: id }))
        .map_err(|e| error::ErrorInternalServerError(e))
}

fn validate_url(req: RegisterRequest, hostname: String) -> Result<RegisterRequest, Error> {
    if (req.url.starts_with("http://") || req.url.starts_with("https://"))
        && !req.url.contains(&hostname)
    {
        Ok(req)
    } else {
        Err(error::ErrorBadRequest(""))
    }
}

#[test]
fn validate_url_test() {
    let f = |s: &str| {
        RegisterRequest {
            url: s.to_string(),
            password: None,
        }
    };

    let req = f("http://example.com");
    let result = validate_url(req, "localhost".to_string());
    assert!(result.is_ok());

    let req = f("https://example.com");
    let result = validate_url(req, "localhost".to_string());
    assert!(result.is_ok());

    let req = f("http://localhost");
    let result = validate_url(req, "localhost".to_string());
    assert!(result.is_err());

    let req = f("tcp://example.com");
    let result = validate_url(req, "localhost".to_string());
    assert!(result.is_err());
}