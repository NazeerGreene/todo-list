use pico_args;
use std::io::{self, Write};
use todo_list::{Task, ToDoList};

const DEFAULT_USER_OUTPUT_FILE: &str = "todolist.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load/create list
    let mut todolist = ToDoList::load_from(DEFAULT_USER_OUTPUT_FILE).unwrap_or(ToDoList::new());

    // operate on list
    let mut args = pico_args::Arguments::from_env();

    let args = Args {
        print: args.contains(["-l", "--list"]),
        add: args.value_from_str(["-a", "--add"]),
        remove: args.value_from_str(["-r", "--remove"]),
        toggle: args.value_from_str(["-t", "--toggle"]),
    };

    if let Ok(new_task) = args.add {
        handle_add(&mut todolist, new_task)
    } else {
    }

    if let Ok(list_idx) = args.toggle {
        handle_toggle(&mut todolist, list_idx - 1); // "user selection" to "vec index"" conversion
    } else {
    }

    if let Ok(list_idx) = args.remove {
        handle_remove(&mut todolist, list_idx - 1); // "user selection" to "vec index"" conversion
    } else {
    }

    if args.print {
        handle_print(&todolist);
    };

    // save and quit
    match todolist.save_to(DEFAULT_USER_OUTPUT_FILE) {
        Ok(_) => println!("Successfully saved to {}", DEFAULT_USER_OUTPUT_FILE),
        Err(err) => {
            eprintln!("Error saving file to {}: {}", DEFAULT_USER_OUTPUT_FILE, err);
            std::process::exit(1);
        }
    }

    Ok(())
}

struct Args {
    print: bool,
    add: Result<String, pico_args::Error>,
    remove: Result<usize, pico_args::Error>,
    toggle: Result<usize, pico_args::Error>,
}

fn handle_print(list: &ToDoList) {
    for (num, item) in list.iter().enumerate() {
        println!("({:02}) {}", num + 1, item);
    }
}

fn handle_toggle(todolist: &mut ToDoList, list_idx: usize) {
    todolist.toggle_task(list_idx);
}

fn handle_remove(todolist: &mut ToDoList, list_idx: usize) {
    let _removed_task = todolist.remove(list_idx);
}

fn handle_add(todolist: &mut ToDoList, new_task: String) {
    todolist.add(Task::from(false, &new_task));
}

fn handle_error(error: pico_args::Error) {
    // let emessage = match error {
    //     pico_args::Error::MissingArgument => "Missing an argument follwing a flag",
    //     pico_args::Error::OptionWithoutAValue(value) => {
    //         format!("Missing the value following {}", value)
    //     }
    // };

    // eprintln!(format!("{}", emessage));
}

fn get_string(prompt: &str) -> io::Result<String> {
    let mut string = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut string)?;

    Ok(String::from(string.trim()))
}
