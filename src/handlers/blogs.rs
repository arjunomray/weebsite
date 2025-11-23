use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use crate::services::blog_service;
use crate::models::blog::BlogPost;

#[derive(Template)]
#[template(path = "blog_list.html")]
pub struct BlogListTemplate {
    pub posts: Vec<BlogPost>,
    pub active_page: String,
}

#[derive(Template)]
#[template(path = "blog_post.html")]
pub struct BlogPostTemplate {
    pub post: BlogPost,
    pub active_page: String,
}

/// Handler for listing all blog posts
pub async fn list_posts_handler() -> impl IntoResponse {
    match blog_service::load_all_posts() {
        Ok(posts) => {
            let template = BlogListTemplate { 
                posts,
                active_page: "blog".to_string()
            };
            (StatusCode::OK, Html(template.render().unwrap()))
        }
        Err(e) => {
            tracing::error!("Failed to load blog posts: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("<h1>Error loading blog posts</h1>".to_string()),
            )
        }
    }
}

/// Handler for viewing a specific blog post
pub async fn view_post_handler(Path(slug): Path<String>) -> impl IntoResponse {
    match blog_service::load_post_by_slug(&slug) {
        Ok(post) => {
            let template = BlogPostTemplate { 
                post,
                active_page: "blog".to_string()
            };
            (StatusCode::OK, Html(template.render().unwrap()))
        }
        Err(e) => {
            tracing::error!("Failed to load blog post '{}': {}", slug, e);
            (
                StatusCode::NOT_FOUND,
                Html("<h1>Blog post not found</h1>".to_string()),
            )
        }
    }
}
