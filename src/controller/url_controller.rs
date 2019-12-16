use super::super::model::url_list;
use super::util;
use actix_web::{
    error, http::StatusCode, Error, HttpRequest, HttpResponse,
    Responder,
    Result,
    web::{
        Query,
        Json,
    },
};
use futures::future::Future;
use std::sync::Arc;
use std::collections::HashMap;
use ApplicationState;

const ID_LEN: usize = 8;

pub fn get_url(req: HttpRequest) -> Result<impl Responder, Error> {
    let path = req.path();
    let id = path.replacen("/", "", 1);
    let q = req.query_string();
    let query: Query<HashMap<String, String>> = Query::from_query(q).unwrap();
    let new_password = util::generate_password_hash(query.get("password").map(|s| s.to_string()), &id);

    let state: Option<&Arc<ApplicationState>> = req.app_data();
    let state = state.unwrap();
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
pub struct RegisterRequest {
    id: Option<String>,
    url: String,
    password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    id: String,
}

pub fn register(
    json: Json<RegisterRequest>,
    req: HttpRequest,
) -> Result<impl Responder, Error> {
    let (pool, hostname) = {
        let state: Option<&Arc<ApplicationState>> = req.app_data();
        let state = state.unwrap();
        (state.pool.clone(), state.hostname.clone())
    };
    
    validate_url(json.0, hostname).and_then(move |v| register_url(v, pool))
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
