use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::app_state;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegList {
    pub list: Vec<common::reg::Reg>,
}

impl Default for RegList {
    fn default() -> Self {
        Self::new()
    }
}

impl RegList {
    pub fn new() -> RegList {
        RegList { list: vec![] }
    }

    fn add(&mut self, reg: common::reg::Reg) {
        self.list.push(reg);
    }
}

pub async fn reg(
    reg: web::Json<common::reg::Reg>,
    data: web::Data<app_state::AppState>,
) -> impl Responder {
    info!(">>> recv: reg {}::{}/{}", reg.ip, reg.port, reg.path);

    let mut reg_list = data.reg_list.lock().unwrap();

    reg_list.add(reg.into_inner());

    HttpResponse::Ok().body("reg")
}

pub async fn print(data: web::Data<app_state::AppState>) -> impl Responder {
    info!(">>> recv: reg::print");

    let reg_list = data.reg_list.lock().unwrap();

    HttpResponse::Ok().body(json!(reg_list.list).to_string())
}
