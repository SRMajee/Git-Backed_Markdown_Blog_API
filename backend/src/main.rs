use std::path::PathBuf;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "git_blog_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Check environment variable first, then fallback to common paths
    let content_dir = if let Ok(custom_path) = std::env::var("CONTENT_DIR") {
        PathBuf::from(custom_path)
    } else if PathBuf::from("./content/posts").exists() {
        PathBuf::from("./content/posts")
    } else if PathBuf::from("./backend/content/posts").exists() {
        PathBuf::from("./backend/content/posts")
    } else {
        let default_dir = PathBuf::from("./content/posts");
        tokio::fs::create_dir_all(&default_dir).await?;
        default_dir
    };

    tracing::info!("📁 Serving posts from: {:?}", content_dir.canonicalize().unwrap_or(content_dir.clone()));

    let app = git_blog_api::create_router(content_dir);

    let addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("🦀 Rust Axum server running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
