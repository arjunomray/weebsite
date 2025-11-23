use axum::{routing::get, Router};
use crate::handlers::projects::list_projects_handler;

pub fn router() -> Router {
    Router::new().route("/projects", get(list_projects_handler))
}
