use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct IndexTemplate<'a> {
    pub name: &'a str,
    pub active_page: String,
}
