use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct IndexTemplate {
    pub active_page: String,
}
