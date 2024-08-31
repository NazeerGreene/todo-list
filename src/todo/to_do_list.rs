use std::fs::File;
use std::io::{BufWriter, Write};

use crate::todo::task::Task;

const DEFAULT_CAPACITY: usize = 10; // for ToDoList underlying vec -- arbitrary

pub struct ToDoList {
    list: Vec<Task>,
}

impl ToDoList {
    pub fn new() -> Self {
        Self {
            list: Vec::with_capacity(DEFAULT_CAPACITY),
        }
    }

    pub fn load_from(filename: &str) -> std::io::Result<Self> {
        let file = File::open(filename)?;
        /*
            1. open file
                - good path: go to read contents
                - bad path: return Result::Error
            2. read contents from file
        */
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

    pub fn save_to(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        for item in self.list.iter() {
            writeln!(
                writer,
                "{}:{}",
                if item.complete { "1" } else { "0" },
                item.task
            )?;
        }

        writer.flush()?;
        Ok(())
    }
}
