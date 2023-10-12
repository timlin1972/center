use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::info;

pub async fn post(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let path = req.uri().to_string();
    let payload: serde_json::Value =
        serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();

    let str = format!(">>> recv: post: path: {path}, payload: {payload}");
    info!("{str}");

    for reg in reg::get_reg_list() {
        if reg.path == path {
            request(&reg, &payload).await.unwrap();
        }
    }

    HttpResponse::Ok().body(str)
}

async fn request(
    reg: &reg::Reg,
    payload: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let uri = format!("http://{}:{}{}", reg.ip, reg.port, reg.path);
    reqwest::Client::new()
        .post(uri)
        .json(payload)
        .send()
        .await?;

    info!(
        ">>> send: post: {}:{}{}, {}",
        reg.ip, reg.port, reg.path, payload
    );

    Ok(())
}
