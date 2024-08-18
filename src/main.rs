use actix_web::{web, App, HttpServer};
use actix_files::Files;
mod app_state;
mod grid;
mod websocket;
mod handlers;
mod markdown_cache;

use app_state::AppState;
use std::env;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let (addr, port) = get_address_and_port();
    let app_state = AppState::new().await;

    info!("Starting server at http://{}:{}", addr, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(Files::new("/static", "./static").show_files_listing())
            .service(
                web::scope("")
                    .route("/", web::get().to(handlers::index))
                    .route("/grid", web::get().to(handlers::get_grid))
                    .route("/switch", web::post().to(handlers::switch_cell))
                    .route("/ws", web::get().to(handlers::grid_socket))
                    .route("/{path}", web::get().to(handlers::serve))
            )
    })
    .bind((addr, port))?
    .run()
    .await
}

fn get_address_and_port() -> (String, u16) {
    let args: Vec<String> = env::args().collect();

    let addr = args.get(1)
        .map(|s| s.to_string())
        .or_else(|| env::var("HOST").ok())
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let port = args.get(2)
        .and_then(|s| s.parse().ok())
        .or_else(|| env::var("PORT").ok().and_then(|s| s.parse().ok()))
        .unwrap_or(8080);

    (addr, port)
}