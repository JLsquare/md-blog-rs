use crate::grid::Grid;
use crate::websocket::GridWebSocket;
use crate::markdown_cache::MarkdownCache;
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashSet;
use actix::Addr;

#[derive(Clone)]
pub struct AppState {
    pub grid: Arc<Mutex<Grid>>,
    pub clients: Arc<Mutex<HashSet<Addr<GridWebSocket>>>>,
    pub markdown_cache: Arc<RwLock<MarkdownCache>>,
}

impl AppState {
    pub async fn new() -> Self {
        let grid = Arc::new(Mutex::new(Grid::load_or_create()));
        let clients = Arc::new(Mutex::new(HashSet::new()));
        let markdown_cache = Arc::new(RwLock::new(MarkdownCache::new()));

        AppState {
            grid,
            clients,
            markdown_cache,
        }
    }
}