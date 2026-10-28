pub mod error;
pub mod handlers;
pub mod models;

use axum::{routing::get, Router};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub content_dir: Arc<PathBuf>,
}

pub fn create_router(content_dir: PathBuf) -> Router {
    let state = AppState {
        content_dir: Arc::new(content_dir),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/posts", get(handlers::post::list_posts))
        .route("/api/posts/:slug", get(handlers::post::get_post_by_slug))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

