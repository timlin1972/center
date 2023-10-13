use std::str::FromStr;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use wol::{send_wol, MacAddr};

use crate::device_list;
#[derive(Serialize, Deserialize, Debug)]
pub struct Wol {
    device: String,
}

pub async fn post(req: HttpRequest, wol: web::Json<Wol>) -> impl Responder {
    let path = req.uri().to_string();
    let wol: Wol = wol.into_inner();

    let str = format!(">>> recv: post: path: {path}, payload: {wol:?}");
    info!("{str}");

    for device in device_list::DEVICE_LIST {
        if device.0 == wol.device {
            info!(">>> wol: {} {}", device.0, device.1);
            send_wol(MacAddr::from_str(device.1).unwrap(), None, None).unwrap();
        }
    }

    HttpResponse::Ok().body(str)
}
