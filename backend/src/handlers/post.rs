use axum::{
    extract::{Path, State},
    Json,
};
use pulldown_cmark::{html, Parser};
use std::path::PathBuf;
use tokio::fs;

use crate::{
    error::AppError,
    models::post::{PostDetail, PostSummary},
    AppState,
};

/// GET /api/posts
/// Reads the markdown directory asynchronously and returns summaries.
pub async fn list_posts(State(state): State<AppState>) -> Result<Json<Vec<PostSummary>>, AppError> {
    let mut entries = fs::read_dir(&*state.content_dir).await?;
    let mut posts = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                let title = file_stem.replace('-', " ");
                posts.push(PostSummary {
                    slug: file_stem.to_string(),
                    title,
                });
            }
        }
    }

    posts.sort_by(|a, b| a.slug.cmp(&b.slug));
    Ok(Json(posts))
}

/// GET /api/posts/:slug
/// Loads a single markdown file, parses it to HTML, and returns the detail.
pub async fn get_post_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<PostDetail>, AppError> {
    // Path traversal guard
    if slug.contains("..") || slug.contains('/') || slug.contains('\\') {
        return Err(AppError::InvalidPath("Invalid slug characters".to_string()));
    }

    let file_path = PathBuf::from(&*state.content_dir).join(format!("{slug}.md"));

    let markdown = fs::read_to_string(&file_path)
        .await
        .map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => AppError::NotFound(slug.clone()),
            _ => AppError::Io(err),
        })?;

    let parser = Parser::new(&markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let title = slug.replace('-', " ");

    Ok(Json(PostDetail {
        slug,
        title,
        markdown,
        html: html_output,
    }))
}

