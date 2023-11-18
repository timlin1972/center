use actix_web::{HttpResponse, Responder};
use log::info;
use log::LevelFilter;

pub mod reg;
pub mod server_log;
pub mod utils;

pub const CENTER_IP: &str = "127.0.0.1";
pub const CENTER_PORT: u16 = 9759;
pub const LOG_LEVEL: LevelFilter = LevelFilter::Info;
pub const LOG_SERVER: &str = "==================================================";
pub const LOG_PLUGIN: &str = "--------------------------------------------------";

pub const LOG_FILE: &str = "server.log";

pub const CONFIG_FILE: &str = "center.toml";

pub const HELLO: &str = "/hello";
pub async fn hello() -> impl Responder {
    info!(">>> recv: hello");
    HttpResponse::Ok().body("Hello")
}
