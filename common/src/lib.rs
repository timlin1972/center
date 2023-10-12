use actix_web::{HttpResponse, Responder};
use log::info;

pub const CENTER_IP: &str = "127.0.0.1";
pub const CENTER_PORT: u16 = 9759;
pub const LOG_LEVEL: &str = "info";

pub const ROOT: &str = "/";
pub async fn root() -> impl Responder {
    let str = ">>> recv: hello";
    info!("{str}");
    HttpResponse::Ok().body(str)
}
