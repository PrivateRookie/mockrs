use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use structopt::StructOpt;

mod api;
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

    /// Listen ip
    #[structopt(long, default_value = "127.0.0.1", env = "MOCKRS_HOST")]
    host: String,

    /// Listen port
    #[structopt(long, default_value = "9000", env = "MOCKRS_PORT")]
    port: usize,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let conf: Config = Config::from_args();
    let db = db::Database::load(&conf.db_file);
    let web_db = web::Data::new(db);
    HttpServer::new(move || {
        App::new()
            .register_data(web_db.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/index").route(web::get().to(api::server_info)))
            .service(web::scope("/_actions").route("/flush", web::post().to(api::flush)))
            .service(
                web::resource("/*")
                    .route(web::get().to(api::do_get))
                    .route(web::post().to(api::do_post))
                    .route(web::put().to(api::do_post))
                    .route(web::delete().to(api::do_delete)),
            )
    })
    .bind(format!("{}:{}", conf.host, conf.port))?
    .start()
    .await
}
