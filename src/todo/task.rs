pub struct Task {
    // this restricts public access to the parent module
    pub(super) complete: bool,
    pub(super) task: String,
}

impl Task {
    pub fn new() -> Self {
        Self {
            complete: false,
            task: String::new(),
        }
    }

    pub fn from(complete: bool, task: &str) -> Self {
        Self {
            complete,
            task: String::from(task),
        }
    }
} // Task

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}",
            if self.complete { "X" } else { " " },
            self.task
        )
    }
}
