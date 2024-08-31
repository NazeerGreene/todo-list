use std::fs::File;
use std::io::Error;
use std::io::{BufWriter, Result, Write};

pub struct Task {
    complete: bool,
    task: String,
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

pub struct ToDoList {
    list: Vec<Task>,
}

impl ToDoList {
    pub fn new() -> Self {
        Self {
            list: Vec::with_capacity(10), // arbitrary capacity
        }
    }

    pub fn add(&mut self, task: Task) {
        self.list.push(task);
    }

    pub fn remove(&mut self, idx: usize) -> Option<Task> {
        if idx <= self.list.capacity() {
            Some(self.list.remove(idx))
        } else {
            None
        }
    }

    pub fn toggle_task(&mut self, idx: usize) {
        if let Some(task) = self.list.get_mut(idx) {
            task.complete = !task.complete;
        }
    }

    // this is the preferred way to return an iterator to an underlying
    // data structure, since the trait is generic so that changing the
    // code doesn't change the public facing API.
    pub fn iter(&self) -> impl Iterator<Item = &Task> + '_ {
        self.list.iter()
    }

    pub fn save_to(&self, filename: &str) -> Result<()> {
        let file = std::fs::File::create(filename)?;
        let mut writer = std::io::BufWriter::new(file);

        for item in self.list.iter() {
            writeln!(
                writer,
                "{}:{}",
                if item.complete { "1" } else { "0" },
                item.task
            );
        }

        writer.flush()?;
        Ok(())
    }
}
