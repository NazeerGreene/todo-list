use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use super::task::Task;

const DEFAULT_CAPACITY: usize = 10; // for ToDoList underlying vec -- arbitrary
const FILE_DELIMITER: &str = ":";

#[derive(Debug)]
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
        if !task.task.is_empty() {
            self.list.push(task);
        }
    }

    pub fn get(&self, idx: usize) -> Option<&Task> {
        self.list.get(idx)
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

#[cfg(test)]
mod tests {

    use super::*;

    // HELPER
    fn get_basic_todo_list() -> ToDoList {
        let mut todo = ToDoList::new();

        todo.add(Task::from(false, "task #1"));
        todo.add(Task::from(false, "task #2"));
        todo.add(Task::from(false, "task #3"));

        todo
    }
    // HELPER

    #[test]
    fn can_add_one_task() {
        let mut todo = ToDoList::new();

        todo.add(Task::from(false, "task #1"));

        assert_eq!(1, todo.size());
    }

    #[test]
    fn doesnt_add_empty_task() {
        let mut todo = ToDoList::new();

        todo.add(Task::from(false, ""));

        assert_eq!(0, todo.size());
    }

    #[test]
    fn removes_one_task() {
        let mut todo = get_basic_todo_list();

        let removed: Option<Task> = todo.remove(0);

        assert_eq!(removed.unwrap().task, "task #1");
        assert_eq!(2, todo.size());
    }

    #[test]
    fn doesnt_remove_task_beyond_bounds() {
        let mut todo = get_basic_todo_list();

        let _removed: Option<Task> = todo.remove(100);

        assert_eq!(3, todo.size());
    }

    #[test]
    fn only_toggles_one_task() {
        let mut todo = get_basic_todo_list();

        todo.toggle_task(0);
        assert_eq!(true, todo.get(0).unwrap().complete);
        assert_eq!(false, todo.get(1).unwrap().complete);
        assert_eq!(false, todo.get(2).unwrap().complete);
    }

    #[test]
    fn saves_to_and_loads_from_file() {
        let output_file = "test-output-todolist.txt";
        let first = get_basic_todo_list();

        first.save_to(output_file);

        let second = ToDoList::load_from(output_file).unwrap();

        // make sure the two lists have the same number of elements
        assert_eq!(first.size(), second.size());

        // each element should be the same, no data modification
        for i in 0..first.size() {
            let firsts_task = first.get(i).unwrap();
            let seconds_task = second.get(i).unwrap();

            assert_eq!(firsts_task.complete, seconds_task.complete);
            assert_eq!(firsts_task.task, seconds_task.task);
        }
    }
}
