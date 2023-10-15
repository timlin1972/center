use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod app_state;
mod auth;
mod plugin;
mod reg;
mod user;

const BINDING_IP: &str = "0.0.0.0";
const BINDING_PORT: u16 = 9759;
const BINDING_HTTPS: &str = "0.0.0.0:9760";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or(common::LOG_LEVEL)).init();

    // app_data
    let reg_list = Arc::new(Mutex::new(reg::RegList::new()));
    let user_list = Arc::new(Mutex::new(user::UserList::new()));

    // https
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(common::CA_PRIVATE_KEY_FILE, SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(common::CA_CERTIFICATE_CHAIN_FILE)
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().send_wildcard();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_state::AppState {
                reg_list: Arc::clone(&reg_list),
                user_list: Arc::clone(&user_list),
            }))
            .route(common::HELLO, web::get().to(common::hello))
            .route(common::reg::PATH, web::post().to(reg::reg))
            .route(common::reg::PRINT, web::get().to(reg::print))
            .route(auth::AUTH, web::post().to(auth::post))
            .route(plugin::PATH, web::get().to(plugin::get))
            .route(plugin::PATH, web::post().to(plugin::post))
            .service(fs::Files::new("/", "../client/build").index_file("index.html"))
    })
    .bind((BINDING_IP, BINDING_PORT))?
    .bind_openssl(BINDING_HTTPS, builder)?
    .run()
    .await
}
