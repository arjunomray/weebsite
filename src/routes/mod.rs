use axum::Router;

mod blogs;
mod home;
mod projects;

pub fn create_routers() -> Router {
    Router::new()
        .merge(home::router())
        .merge(projects::router())
        .merge(blogs::router())
}
