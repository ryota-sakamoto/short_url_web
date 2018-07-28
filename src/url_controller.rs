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
use crypto::{
    sha2::Sha256,
    digest::Digest,
};

const ID_LEN: usize = 8;

pub fn get_url(req: HttpRequest<Arc<ApplicationState>>) -> Result<impl Responder, Error> {
    let path = req.path();
    let id = path.replacen("/", "", 1);

    let state = req.state();
    let url_opt: Option<String> = state.pool.first_exec(r"
        select url from url_list where id = :id
    ", params!{
        "id" => id,
    }).map(|r| {
        r.map(::mysql::from_row)
    }).map_err(|e| {
        println!("[Error]{:?}", e);
        error::ErrorInternalServerError("")
    })?;

    Ok(match url_opt {
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

// TODO transaction
fn register_url(req: RegisterRequest, pool: ::mysql::Pool) -> Result<impl Responder, Error> {
    let id = generate_id();
    let new_password = req.password.map(|s| s + &id.clone());
    let url = req.url;

    let stat = pool.prepare(if new_password.is_some() {
        "insert into url_list values(:id, :password, :url)"
    } else {
        "insert into url_list values(:id, null, :url)"
    });

    match stat {
        Ok(mut s) => {
            s.execute(params!{
                "id" => id.clone(),
                "password" => if let Some(p) = new_password {
                    sha256(&p)
                } else {
                    "".to_string()
                },
                "url" => url
            }).map_err(|e| {
                println!("{}", e);
                error::ErrorInternalServerError(e)
            }).map(|_| {
                Ok(HttpResponse::Ok().json(RegisterResponse {
                    id: id,
                }))
            })?
        },
        Err(e) => {
            Err(error::ErrorInternalServerError(e))
        },
    }
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

fn sha256(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(s);
    hasher.result_str()
}

pub fn remove_url(_: HttpRequest) -> impl Responder {
    HttpResponse::new(StatusCode::NOT_IMPLEMENTED)
}