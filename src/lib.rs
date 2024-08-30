pub mod fillers;

pub mod to_do {

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

    /*  data structure to hold tasks

        TaskList:
            vector: Task

            methods:
                init
                    - new empty vector -- done
                    - vector from list of tasks -- add when needed
                    - vector from another list  -- add when needed
                enable iterator for tasks
                add tasks -- done
                remove tasks -- done
                toggle task completetion -- done
    */
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

        pub fn iter(&self) -> impl Iterator<Item = &Task> + '_ {
            self.list.iter()
        }

        // testing
        // pub fn print(&self) {
        //     for item in self.list.iter() {
        //         println!("[{}] {}", if item.complete { "X" } else { " " }, item.task);
        //     }
        // }
    } // ToDoList
}
