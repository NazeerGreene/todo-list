use todo_list::fillers::{todo, Thing};
use todo_list::{Task, ToDoList};

fn main() {
    todo("finish the rest of the code");
}

fn print_list(list: &ToDoList) {
    for item in list.iter() {
        println!("{}", item);
    }
}
