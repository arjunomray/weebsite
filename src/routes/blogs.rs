use axum::{routing::get, Router};
use crate::handlers::blogs::{list_posts_handler, view_post_handler};

pub fn router() -> Router {
    Router::new()
        .route("/blogs", get(list_posts_handler))
        .route("/blogs/{slug}", get(view_post_handler))
}
