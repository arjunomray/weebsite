use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogMetadata {
    pub title: String,
    pub date: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub description: Option<String>,
    pub content: String,
}

impl BlogPost {
    pub fn new(slug: String, metadata: BlogMetadata, content: String) -> Self {
        Self {
            slug,
            title: metadata.title,
            date: metadata.date,
            description: metadata.description,
            content,
        }
    }
}
