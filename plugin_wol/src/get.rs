use actix_web::{web, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::device_list;

pub const PATH_GET: &str = "/wol";

#[derive(Serialize, Deserialize, Debug)]
struct WolDevice {
    name: String,
    mac: String,
    ip: String,
}

pub async fn get() -> impl Responder {
    info!(">>> recv: get");

    let wol_devices: Vec<WolDevice> = device_list::DEVICE_LIST
        .iter()
        .map(|x| WolDevice {
            name: x.0.to_owned(),
            mac: x.1.to_owned(),
            ip: x.2.to_owned(),
        })
        .collect();

    web::Json(json!(wol_devices))
}
