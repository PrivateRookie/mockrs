use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer};
use structopt::StructOpt;

mod db;

#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "mockrs",
    version = "0.1.0",
    author = "PrivateRookie <996514515@qq.com>",
    about = "a mock restful json http server"
)]
struct Config {
    /// json file as database
    #[structopt(required = true, env = "MOCKRS_DB_FILE")]
    db_file: String,

    /// listen ip
    #[structopt(long, default_value = "127.0.0.1", env = "MOCKRS_HOST")]
    host: String,

    /// listen port
    #[structopt(long, default_value = "9000", env = "MOCKRS_PORT")]
    port: usize,
}

fn index() -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello"))
}

fn db_serve(req: HttpRequest, data: web::Data<db::Database>) -> HttpResponse {
    let mut database = data.data.lock().unwrap();
    let mut keys: Vec<String> = req
        .path()
        .split("/")
        .skip(1)
        .map(|seg| seg.to_string())
        .collect();
    if keys == vec![""] {
        keys = vec![];
    };
    let res = db::Database::get(&mut keys, &mut database);
    match res {
        Ok(obj) => HttpResponse::Ok().json(obj),
        Err(e) => HttpResponse::build(http::StatusCode::BAD_REQUEST).json(e),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let conf: Config = Config::from_args();
    let db = db::Database::load(&conf.db_file);
    let web_db = web::Data::new(db);
    HttpServer::new(move || {
        App::new()
            .register_data(web_db.clone())
            .service(web::resource("/index").route(web::get().to(index)))
            .service(web::resource("/*").route(web::get().to(db_serve)))
    })
    .bind(format!("{}:{}", conf.host, conf.port))?
    .start()
    .await
}
