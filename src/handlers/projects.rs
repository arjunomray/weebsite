use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use crate::services::project_service;
use crate::models::project::Project;

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {
    pub projects: Vec<Project>,
    pub active_page: String,
}

/// Handler for listing all projects
pub async fn list_projects_handler() -> impl IntoResponse {
    let projects = project_service::get_projects();
    let template = ProjectsTemplate { 
        projects,
        active_page: "projects".to_string()
    };
    (StatusCode::OK, Html(template.render().unwrap()))
}
