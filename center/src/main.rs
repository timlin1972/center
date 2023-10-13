use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
use env_logger::Env;

mod app_state;
mod plugin;
mod reg;

const BINDING_IP: &str = "0.0.0.0";
const BINGING_PORT: u16 = 9759;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or(common::LOG_LEVEL)).init();

    let reg_list = Arc::new(Mutex::new(reg::RegList::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state::AppState {
                reg_list: Arc::clone(&reg_list),
            }))
            .route(common::ROOT, web::get().to(common::root))
            .route(common::reg::PATH, web::post().to(reg::reg))
            .route(common::reg::PRINT, web::get().to(reg::print))
            .route(plugin::PATH, web::get().to(plugin::get))
            .route(plugin::PATH, web::post().to(plugin::post))
    })
    .bind((BINDING_IP, BINGING_PORT))?
    .run()
    .await
}
