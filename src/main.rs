use todo_list::ToDoList;

const DEFAULT_USER_OUTPUT_FILE: &str = "todolist.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut todolist = ToDoList::load_from(DEFAULT_USER_OUTPUT_FILE).unwrap_or(ToDoList::new());

    todolist = todo_list::run(todolist);

    if let Err(err) = todolist.save_to(DEFAULT_USER_OUTPUT_FILE) {
        eprintln!("Error saving file to {}: {}", DEFAULT_USER_OUTPUT_FILE, err);
        std::process::exit(1);
    }

    Ok(())
}
