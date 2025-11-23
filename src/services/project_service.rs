use crate::models::project::Project;

pub fn get_projects() -> Vec<Project> {
    vec![
        Project::new(
            "Rustime".to_string(),
            "https://github.com/arjunomray/rustime".to_string(),
            "This is a rust based cli pomodoro timer".to_string(),
        ),
        Project::new(
            "Grab a Seat".to_string(),
            "https://github.com/arjunomray/Mailer".to_string(),
            "This is an automation script which sends a bunch of referral mails to companies".to_string(),
        ),
        Project::new(
            "Task Master".to_string(),
            "https://github.com/arjun/task-master".to_string(),
            "A CLI task management tool built with Rust. Helps organize daily tasks with priorities and due dates.".to_string(),
        ),
    ]
}
