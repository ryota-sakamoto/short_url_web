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
    let new_password = util::generate_password_hash(q.get("password").map(|s| s.to_string()), &id);

    let state = req.state();
    let url_result = url_list::find(&state.pool, id, new_password);

    url_result
        .map(|url_opt| match url_opt {
            Some(u) => HttpResponse::Ok()
                .status(StatusCode::MOVED_PERMANENTLY)
                .header("Location", u.url)
                .finish(),
            None => HttpResponse::Ok().status(StatusCode::NOT_FOUND).finish(),
        })
        .map_err(|e| error::ErrorInternalServerError(e))
}

#[derive(Debug, Serialize, Deserialize)]
struct RegisterRequest {
    id: Option<String>,
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
    let id = req.id.unwrap_or(util::generate_id(ID_LEN));
    let new_password = util::generate_password_hash(req.password, &id);
    let url = req.url;

    url_list::find(&pool, id.clone(), new_password.clone())
        .map(|url_opt| match url_opt {
            Some(_) => Err(error::ErrorBadRequest(format!("Already Exists ID: {}", id))),
            None => url_list::insert(&pool, &id, new_password, url)
                .map(|_| HttpResponse::Ok().json(RegisterResponse { id: id }))
                .map_err(|e| error::ErrorInternalServerError(e)),
        })
        .map_err(|e| error::ErrorInternalServerError(e))?
}

fn validate_url(req: RegisterRequest, hostname: String) -> Result<RegisterRequest, Error> {
    if (req.url.starts_with("http://") || req.url.starts_with("https://"))
        && (!req.url.starts_with(&format!("http://{}", hostname))
            && !req.url.starts_with(&format!("https://{}", hostname)))
    {
        Ok(req)
    } else {
        Err(error::ErrorBadRequest(format!("Invalid URL: {}", req.url)))
    }
}

#[test]
fn validate_url_test() {
    let f = |s: &str| RegisterRequest {
        id: None,
        url: s.to_string(),
        password: None,
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

    let req = f("https://localhost");
    let result = validate_url(req, "localhost".to_string());
    assert!(result.is_err());

    let req = f("http://example.com/localhost");
    let result = validate_url(req, "localhost".to_string());
    assert!(result.is_ok());

    let req = f("tcp://example.com");
    let result = validate_url(req, "localhost".to_string());
    assert!(result.is_err());
}
