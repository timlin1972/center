use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use log::info;

const BINDING_IP: &str = "127.0.0.1";
const BINGING_PORT: u16 = 9760;

const PATH: &str = "/wol";
const PORT: u16 = BINGING_PORT;

async fn request() -> Result<(), Box<dyn std::error::Error>> {
    let reg = reg::Reg {
        ip: BINDING_IP.to_owned(),
        path: PATH.to_owned(),
        port: PORT,
    };

    reqwest::Client::new()
        .post(format!(
            "http://{}:{}{}",
            common::CENTER_IP,
            common::CENTER_PORT,
            reg::PATH
        ))
        .json(&reg)
        .send()
        .await?;

    info!("{}", format!(">>> send: reg {BINDING_IP}::{PORT}{PATH}"));

    Ok(())
}

pub async fn post(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let path = req.uri().to_string();
    let payload: serde_json::Value =
        serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();

    let str = format!(">>> recv: post: path: {path}, payload: {payload}");
    info!("{str}");

    HttpResponse::Ok().body(str)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or(common::LOG_LEVEL)).init();

    request().await.unwrap();

    HttpServer::new(|| {
        App::new()
            .route(common::ROOT, web::get().to(common::root))
            .route(PATH, web::post().to(post))
    })
    .bind((BINDING_IP, BINGING_PORT))?
    .run()
    .await
}
