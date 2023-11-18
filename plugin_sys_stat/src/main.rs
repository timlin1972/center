use actix_web::{web, App, HttpServer};
use log::info;

mod get;

const BINDING_IP: &str = "127.0.0.1";
const BINGING_PORT: u16 = 9762;

const PATH_GET: &str = "sys_stat";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // logger
    common::server_log::init();
    info!("{}", common::LOG_PLUGIN);

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
