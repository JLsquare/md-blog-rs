use actix_web::{web, HttpResponse, Result, Responder, HttpRequest};
use crate::app_state::AppState;
use crate::websocket::{GridWebSocket, GridUpdate};
use actix_web_actors::ws;
use log::{info, warn};
use std::fs;

const TEMPLATE: &str = include_str!("../index.html");

pub async fn index(data: web::Data<AppState>) -> Result<HttpResponse> {
    serve_markdown("index", data).await
}

pub async fn serve(path: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse> {
    serve_markdown(&path, data).await
}

async fn serve_markdown(path: &str, data: web::Data<AppState>) -> Result<HttpResponse> {
    info!("Serving markdown file: {}", path);

    let file_path = format!("./markdown/{}.md", path);
    
    if let Ok(metadata) = fs::metadata(&file_path) {
        let last_modified = metadata.modified().unwrap_or(std::time::SystemTime::now());
        
        let cache = data.markdown_cache.read().unwrap();
        if let Some(cached) = cache.cache.get(path) {
            if cached.last_modified >= last_modified {
                return Ok(HttpResponse::Ok().content_type("text/html").body(cached.html.clone()));
            }
        }
        drop(cache);

        let markdown_content = fs::read_to_string(&file_path)?;
        let html_content = markdown::to_html(&markdown_content);
        let rendered = TEMPLATE.replace("{{ markdown_content }}", &html_content);

        let mut cache = data.markdown_cache.write().unwrap();
        cache.cache.insert(path.to_string(), crate::markdown_cache::CachedMarkdown {
            html: rendered.clone(),
            last_modified,
        });

        Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
    } else {
        warn!("File not found: {}", path);
        Ok(HttpResponse::NotFound().finish())
    }
}

pub async fn get_grid(data: web::Data<AppState>) -> impl Responder {
    let grid = data.grid.lock().unwrap();
    web::Json(grid.clone())
}

pub async fn switch_cell(data: web::Data<AppState>, info: web::Json<(usize, usize)>) -> impl Responder {
    let (x, y) = info.into_inner();
    let mut grid = data.grid.lock().unwrap();
    grid.switch(x, y);

    if let Err(e) = grid.save_to_file() {
        warn!("Failed to save grid state: {}", e);
    }
    
    let clients = data.clients.lock().unwrap();
    let grid_json = serde_json::to_string(&*grid).unwrap();
    for client in clients.iter() {
        client.do_send(GridUpdate(grid_json.clone()));
    }
    
    HttpResponse::Ok().finish()
}

pub async fn grid_socket(req: HttpRequest, stream: web::Payload, data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    ws::start(
        GridWebSocket {
            clients: data.clients.clone(),
        },
        &req,
        stream,
    )
}