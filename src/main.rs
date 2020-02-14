use actix_web::{http, middleware, web, App, HttpServer};
use dotenv::dotenv;
use jen::generator::Generator;
use std::io::{prelude::*, Error, ErrorKind};
use structopt::StructOpt;

mod api;
mod db;

#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = "PrivateRookie <996514515@qq.com>",
    about = env!("CARGO_PKG_DESCRIPTION")
)]
enum Config {
    /// Run http json server
    Serve {
        /// Json file as database
        #[structopt(required = true, env = "MOCKRS_DB_FILE")]
        db_file: String,

        /// Listen ip
        #[structopt(long, default_value = "127.0.0.1", env = "MOCKRS_HOST")]
        host: String,

        /// Listen port
        #[structopt(long, default_value = "9000", env = "MOCKRS_PORT")]
        port: usize,
    },

    /// Generate fake data based on template
    Gen {
        /// Template file to generate json file
        #[structopt(required = true)]
        template: String,

        /// Output json file
        #[structopt(long)]
        output: Option<String>,
    },
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    match Config::from_args() {
        Config::Serve {
            db_file,
            host,
            port,
        } => {
            let db = db::Database::load(&db_file);
            let web_db = web::Data::new(db);
            HttpServer::new(move || {
                App::new()
                    .app_data(web_db.clone())
                    .wrap(middleware::Logger::default())
                    .service(web::resource("/index").route(web::get().to(api::server_info)))
                    .service(web::scope("/_actions").route("/flush", web::post().to(api::flush)))
                    .service(
                        web::resource("/*")
                            .route(web::get().to(api::do_get))
                            .route(web::method(http::Method::OPTIONS).to(api::do_options))
                            .route(web::post().to(api::do_post))
                            .route(web::put().to(api::do_post))
                            .route(web::delete().to(api::do_delete)),
                    )
            })
            .bind(format!("{}:{}", host, port))?
            .run()
            .await
        }
        Config::Gen { template, output } => match Generator::new(template) {
            Err(_) => Err(Error::new(ErrorKind::NotFound, "can not find template")),
            Ok(mut gen) => {
                match output {
                    None => println!("{}", gen.create()),
                    Some(output) => {
                        let mut f = std::fs::File::create(output)?;
                        f.write(gen.create().as_bytes())?;
                    }
                }
                Ok(())
            }
        },
    }
}
