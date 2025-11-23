use crate::models::project::Project;

pub fn get_projects() -> Vec<Project> {
    vec![
        Project::new(
            "Weeb Site".to_string(),
            "https://github.com/arjun/weeb-site".to_string(),
            "My personal portfolio and blog built with Rust, Axum, and Askama. Features markdown blog posts and a clean, minimal design.".to_string(),
        ),
        Project::new(
            "Rusty Raytracer".to_string(),
            "https://github.com/arjun/rusty-raytracer".to_string(),
            "A ray tracer written in Rust following the 'Ray Tracing in One Weekend' book series. Supports spheres, materials, and defocus blur.".to_string(),
        ),
        Project::new(
            "Task Master".to_string(),
            "https://github.com/arjun/task-master".to_string(),
            "A CLI task management tool built with Rust. Helps organize daily tasks with priorities and due dates.".to_string(),
        ),
    ]
}
