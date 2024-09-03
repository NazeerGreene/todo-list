use pico_args::Arguments;
use std::io::{self, Write};

use todo_list::{Task, ToDoList};

const DEFAULT_USER_OUTPUT_FILE: &str = "todolist.txt";

fn main() {
    // load/create list
    let mut todolist = ToDoList::load_from(DEFAULT_USER_OUTPUT_FILE).unwrap_or(ToDoList::new());

    // operate on list
    // list
    // add
    // remove
    // toggle
    let mut args = pico_args::Arguments::from_env();

    let args = Args {
        print: args.contains(["-l", "--list"]),
        remove: args.value_from_str(["-r", "--remove"]),
        toggle: args.value_from_str(["-t", "--toggle"]),
    };

    if args.print {
        print_list(&todolist)
    };

    // save and quit
    match todolist.save_to(DEFAULT_USER_OUTPUT_FILE) {
        Ok(_) => println!("Successfully saved to {}", DEFAULT_USER_OUTPUT_FILE),
        Err(err) => {
            eprintln!("Error saving file to {}: {}", DEFAULT_USER_OUTPUT_FILE, err);
            std::process::exit(1);
        }
    }
}

struct Args {
    print: bool,
    // add: bool,
    remove: Result<usize, pico_args::Error>,
    toggle: Result<usize, pico_args::Error>,
}

fn print_list(list: &ToDoList) {
    for item in list.iter() {
        println!("{}", item);
    }
}

fn get_string(prompt: &str) -> io::Result<String> {
    let mut string = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut string)?;

    Ok(String::from(string.trim()))
}

fn get_bool(prompt: &str) -> io::Result<bool> {
    let bool_string = get_string(prompt)?;

    match bool_string.as_str() {
        "y" | "yes" | "1" => Ok(true),
        _ => Ok(false),
    }
}
