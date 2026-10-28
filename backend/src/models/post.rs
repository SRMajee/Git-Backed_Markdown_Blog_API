use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostSummary {
    pub slug: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostDetail {
    pub slug: String,
    pub title: String,
    pub markdown: String,
    pub html: String,
}

