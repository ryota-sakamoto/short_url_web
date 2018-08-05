#[derive(Debug)]
pub struct URL {
    pub url: String,
}

impl URL {
    fn new(url: String) -> Self {
        URL { url: url }
    }
}

pub fn find(
    pool: &::mysql::Pool,
    id: String,
    password: Option<String>,
) -> Result<Option<URL>, ::mysql::Error> {
    pool.first_exec(
        if password.is_some() {
            "select url from url_list where id = :id and password = :password"
        } else {
            "select url from url_list where id = :id and password is null"
        },
        params!{
            "id" => id,
            "password" => password.unwrap_or(String::new()),
        },
    ).map(|r| {
        r.map(|r| {
            let row_url = ::mysql::from_value(r[0].clone());
            let u = String::from_utf8(row_url).unwrap();
            URL::new(u)
        })
    })
}

pub fn insert(
    pool: &::mysql::Pool,
    id: &str,
    password: Option<String>,
    url: String,
) -> Result<(), ::mysql::Error> {
    let stat = pool.prepare(if password.is_some() {
        "insert into url_list values(:id, :password, :url)"
    } else {
        "insert into url_list values(:id, null, :url)"
    });

    match stat {
        Ok(mut s) => s.execute(params!{
            "id" => id,
            "password" => password.unwrap_or(String::new()),
            "url" => url
        }).map(|_| Ok(()))?,
        Err(e) => Err(e),
    }
}
