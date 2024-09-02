use todo_list::{Task, ToDoList};

fn main() {}

fn print_list(list: &ToDoList) {
    for item in list.iter() {
        println!("{}", item);
    }
}
