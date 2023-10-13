use actix_web::{web, App, HttpServer};
use env_logger::Env;
use log::info;

mod get;
mod post;
mod device_list;

const BINDING_IP: &str = "127.0.0.1";
const BINGING_PORT: u16 = 9760;

const PATH: &str = "wol";
const PORT: u16 = BINGING_PORT;

const PATH_GET: &str = "/wol/{path:.*}";

async fn request() -> Result<(), Box<dyn std::error::Error>> {
    let reg = common::reg::Reg {
        ip: BINDING_IP.to_owned(),
        path: PATH.to_owned(),
        port: PORT,
    };

    reqwest::Client::new()
        .post(format!(
            "http://{}:{}{}",
            common::CENTER_IP,
            common::CENTER_PORT,
            common::reg::PATH
        ))
        .json(&reg)
        .send()
        .await?;

    info!(">>> send: reg {BINDING_IP}::{PORT}{PATH}");

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or(common::LOG_LEVEL)).init();

    request().await.unwrap();

    HttpServer::new(|| {
        App::new()
            .route(common::ROOT, web::get().to(common::root))
            .route(PATH_GET, web::get().to(get::get))
            .route(PATH, web::post().to(post::post))
    })
    .bind((BINDING_IP, BINGING_PORT))?
    .run()
    .await
}
