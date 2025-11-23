use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::services::templating::IndexTemplate;

pub async fn home_handler() -> impl IntoResponse {
    let template = IndexTemplate {
        active_page: "home".to_string(),
    };
    (StatusCode::OK, Html(template.render().unwrap()))
}
