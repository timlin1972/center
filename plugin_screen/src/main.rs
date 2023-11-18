use actix_web::{web, App, HttpServer};
use log::info;

mod post;

const BINDING_IP: &str = "127.0.0.1";
const BINGING_PORT: u16 = 9764;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // logger
    common::server_log::init();
    info!("{}", common::LOG_PLUGIN);

    common::reg::request(BINDING_IP, BINGING_PORT, post::PATH_POST)
        .await
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .route(common::HELLO, web::get().to(common::hello))
            .route(post::PATH_POST, web::post().to(post::post))
    })
    .bind((BINDING_IP, BINGING_PORT))?
    .run()
    .await
}
