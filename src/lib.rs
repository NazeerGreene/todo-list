use pico_args;

mod todo;

pub use todo::task::Task;
pub use todo::to_do_list::ToDoList;

pub fn run(mut todolist: ToDoList) -> ToDoList {
    let mut args = pico_args::Arguments::from_env();

    let args = Args {
        print: args.contains(["-l", "--list"]),
        add: args.value_from_str(["-a", "--add"]),
        remove: args.value_from_str(["-r", "--remove"]),
        toggle: args.value_from_str(["-t", "--toggle"]),
        // unregonized: args.finish(),
    };

    // if !args.unregonized.is_empty() {
    //     eprintln!("Warning: Unrecognized arguments: {:?}", args.unregonized);
    //     std::process::exit(2);
    // }

    if let Ok(new_task) = args.add {
        handle_add(&mut todolist, new_task)
    } else {
        handle_error(args.add.unwrap_err());
    }

    if let Ok(list_idx) = args.toggle {
        handle_toggle(&mut todolist, list_idx - 1); // "user selection" to "vec index"" conversion
    } else {
        handle_error(args.toggle.unwrap_err());
    }

    if let Ok(list_idx) = args.remove {
        handle_remove(&mut todolist, list_idx - 1); // "user selection" to "vec index"" conversion
    } else {
        handle_error(args.remove.unwrap_err());
    }

    if args.print {
        handle_print(&todolist);
    };

    todolist
}

struct Args {
    print: bool,
    add: Result<String, pico_args::Error>,
    remove: Result<usize, pico_args::Error>,
    toggle: Result<usize, pico_args::Error>,
    // unregonized: Vec<std::ffi::OsString>,
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
    match error {
        pico_args::Error::OptionWithoutAValue(value) => {
            eprintln!("{} requires an accompaning value", value)
        }
        _ => (),
    };
}
