use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use crate::todo::task::Task;

const DEFAULT_CAPACITY: usize = 10; // for ToDoList underlying vec -- arbitrary
const FILE_DELIMITER: &str = ":";

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
        let mut todolist = ToDoList::new();

        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            // unwraps and handles errors
            /* The above is equivalent to:
                let line = match line {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                };
            */

            if let Some((completed_str, task)) = line.split_once(FILE_DELIMITER) {
                todolist.add(Task::from(completed_str == "1", task));
            } // else, skip
        }

        Ok(todolist)
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

    pub fn size(&self) -> usize {
        self.list.len()
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
