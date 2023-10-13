use std::str::FromStr;

use actix_web::{web, HttpResponse, Responder};
use log::info;
use wol::{send_wol, MacAddr};

use crate::device_list;

pub async fn get(
    path: web::Path<String>,
) -> impl Responder {
    let path: String = path.into_inner();

    let str = format!(">>> recv: get: path: {path}");
    info!("{str}");

    for device in device_list::DEVICE_LIST {
        if device.0 == path {
            info!(">>> wol: {} {}", device.0, device.1);
            send_wol(MacAddr::from_str(device.1).unwrap(), None, None).unwrap();
        }
    }

    HttpResponse::Ok().body(str)
}