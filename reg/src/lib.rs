use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub const PATH: &str = "/reg";
pub const PRINT: &str = "/reg/print";

#[derive(Serialize, Deserialize)]
pub struct Reg {
    pub ip: String,
    pub port: u16,
    pub path: String,
}

pub async fn reg(reg: web::Json<Reg>) -> impl Responder {
    info!(">>> recv: reg {}::{}{}", reg.ip, reg.port, reg.path);

    HttpResponse::Ok().body("reg")
}

pub fn get_reg_list() -> Vec<Reg> {
    vec![Reg {
        ip: "127.0.0.1".to_owned(),
        path: "/wol".to_owned(),
        port: 9760,
    }]
}

pub async fn print() -> impl Responder {
    info!(">>> recv: print");

    let reg_list = json!(get_reg_list());

    HttpResponse::Ok().body(reg_list.to_string())
}
