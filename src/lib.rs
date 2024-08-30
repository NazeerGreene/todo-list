pub mod fillers;

pub mod ToDo {

    struct Task {
        complete: bool,
        task: String,
    }

    impl Task {
        fn new() -> Self {
            Task {
                complete: false,
                task: String::new(),
            }
        }

        fn from(complete: bool, task: String) -> Self {
            Task { complete, task }
        }
    } // Task

    /*  data structure to hold tasks

        TaskList:
            vector: Task

            methods:
                init
                    - new empty vector
                    - vector from list of tasks
                    - vector from another list
                print tasks
                add tasks
                remove tasks
                toggle task completetion
    */
}
