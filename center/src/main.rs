use actix_web::{web, App, HttpServer};
use env_logger::Env;

mod post;

const BINDING_IP: &str = "0.0.0.0";
const BINGING_PORT: u16 = 9759;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or(common::LOG_LEVEL)).init();

    HttpServer::new(|| {
        let mut app = App::new()
            .route(common::ROOT, web::get().to(common::root))
            .route(reg::PATH, web::post().to(reg::reg))
            .route(reg::PRINT, web::get().to(reg::print));

        for reg in reg::get_reg_list() {
            app = app.route(&reg.path, web::post().to(post::post));
        }
        app
    })
    .bind((BINDING_IP, BINGING_PORT))?
    .run()
    .await
}
