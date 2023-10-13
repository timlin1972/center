use actix_web::{web, HttpResponse, Responder};
use log::info;

use crate::app_state;

pub const PATH: &str = "/plugin/{path:.*}";

async fn request_get(
    path: &str,
    reg: &common::reg::Reg,
) -> Result<(), Box<dyn std::error::Error>> {
    let uri = format!("http://{}:{}/{}", reg.ip, reg.port, path);
    reqwest::Client::new()
        .get(uri)
        .send()
        .await?;

    info!(
        ">>> send: get: {}:{}/{}",
        reg.ip, reg.port, path
    );

    Ok(())
}

async fn request_post(
    reg: &common::reg::Reg,
    payload: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let uri = format!("http://{}:{}/{}", reg.ip, reg.port, reg.path);
    reqwest::Client::new()
        .post(uri)
        .json(payload)
        .send()
        .await?;

    info!(
        ">>> send: post: {}:{}/{}, {}",
        reg.ip, reg.port, reg.path, payload
    );

    Ok(())
}

pub async fn get(
    path: web::Path<String>,
    data: web::Data<app_state::AppState>,
) -> impl Responder {
    let path: String = path.into_inner();

    let str = format!(">>> recv: get: path: {path}");
    info!("{str}");

    let reg_list_list = {
        let reg_list = data.reg_list.lock().unwrap();
        reg_list.list.clone()
    };
    for reg in &reg_list_list {
        if path.starts_with(&reg.path) {
            request_get(&path, reg).await.unwrap();
        }
    }

    HttpResponse::Ok().body(str)
}

pub async fn post(
    payload: String,
    path: web::Path<String>,
    data: web::Data<app_state::AppState>,
) -> impl Responder {
    let path: String = path.into_inner();
    let payload: serde_json::Value = serde_json::from_str(&payload).unwrap();

    let str = format!(">>> recv: post: path: {path}, payload: {payload}");
    info!("{str}");

    let reg_list_list = {
        let reg_list = data.reg_list.lock().unwrap();
        reg_list.list.clone()
    };
    for reg in &reg_list_list {
        if path.starts_with(&reg.path) {
            request_post(reg, &payload).await.unwrap();
        }
    }

    HttpResponse::Ok().body(str)
}
