use actix_web::{web, App, HttpServer};
use env_logger::Env;

mod get;

const BINDING_IP: &str = "127.0.0.1";
const BINGING_PORT: u16 = 9763;

const PATH_GET: &str = "shutdown";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or(common::LOG_LEVEL)).init();

    common::reg::request(BINDING_IP, BINGING_PORT, PATH_GET)
        .await
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .route(common::HELLO, web::get().to(common::hello))
            .route(PATH_GET, web::get().to(get::get))
    })
    .bind((BINDING_IP, BINGING_PORT))?
    .run()
    .await
}
