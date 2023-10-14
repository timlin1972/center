use actix_web::{HttpResponse, Responder};
use log::info;

pub mod reg;

pub const CENTER_IP: &str = "127.0.0.1";
pub const CENTER_PORT: u16 = 9759;
pub const LOG_LEVEL: &str = "info";

pub const CA_PRIVATE_KEY_FILE: &str = "/home/timlin/ca/privatekey.pem";
pub const CA_CERTIFICATE_CHAIN_FILE: &str = "/home/timlin/ca/certificate.pem";

pub const HELLO: &str = "/hello";
pub async fn hello() -> impl Responder {
    info!(">>> recv: hello");
    HttpResponse::Ok().body("Hello")
}
