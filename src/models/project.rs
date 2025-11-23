use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub repo_link: String,
    pub description: String,
}

impl Project {
    pub fn new(name: String, repo_link: String, description: String) -> Self {
        Self {
            name,
            repo_link,
            description,
        }
    }
}
