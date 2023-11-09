use actix_web::{HttpResponse, Responder};
use log::{error, info};

use common::utils;

pub async fn get() -> impl Responder {
    info!(">>> recv: get");

    match utils::run_command("shutdown", &["-h", "now"]) {
        Ok(_) => HttpResponse::Ok().body("Ok"),
        Err(e) => {
            error!(">>> Failed to run shutdown: {e:?}");

            match utils::run_command("sudo", &["shutdown", "-h", "now"]) {
                Ok(_) => HttpResponse::Ok().body("Ok"),
                Err(e) => {
                    error!(">>> Failed to run sudo shutdown: {e:?}");
                    HttpResponse::Ok().body("Failed")
                }
            }
        }
    }
}
