use axum::{routing::get, Router};

use crate::handlers::home::home_handler;

pub fn router() -> Router {
    Router::new().route("/", get(home_handler))
}
