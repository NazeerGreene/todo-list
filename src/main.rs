use todo_list::ToDoList;

const DEFAULT_USER_OUTPUT_FILE: &str = "todolist.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load/create list
    let mut todolist = ToDoList::load_from(DEFAULT_USER_OUTPUT_FILE).unwrap_or(ToDoList::new());

    // operate on list
    todolist = todo_list::run(todolist);

    // save and quit
    if let Err(err) = todolist.save_to(DEFAULT_USER_OUTPUT_FILE) {
        eprintln!("Error saving file to {}: {}", DEFAULT_USER_OUTPUT_FILE, err);
        std::process::exit(1);
    }

    Ok(())
}
