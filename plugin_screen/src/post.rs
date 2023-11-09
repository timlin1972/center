use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::{error, info};
use serde::{Deserialize, Serialize};

use common::utils;

pub const PATH_POST: &str = "screen";

#[derive(Serialize, Deserialize, Debug)]
pub struct Screen {
    action: String,
}

fn action_screen(action: &str) -> Result<(), std::io::Error> {
    utils::run_command("xset", &["-display", ":0.0", "dpms", "force", action])
}

pub async fn post(req: HttpRequest, screen: web::Json<Screen>) -> impl Responder {
    let path = req.uri().to_string();
    let screen: Screen = screen.into_inner();

    info!(">>> recv: post: path: {path}, payload: {screen:?}");

    match screen.action.as_str() {
        "on" | "off" => match action_screen(&screen.action) {
            Ok(_) => HttpResponse::Ok().body("Ok"),
            Err(_) => {
                error!(">>> Failed to set screen {}", screen.action);
                HttpResponse::Ok().body("Failed")
            }
        },
        _ => {
            error!(">>> unknown action: {}", screen.action);
            HttpResponse::Ok().body("Failed")
        }
    }
}
