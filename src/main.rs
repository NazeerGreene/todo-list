use todo_list::fillers::{todo, Thing};
use todo_list::to_do::{Task, ToDoList};

fn main() {
    let mut mylist = ToDoList::new();

    mylist.add(Task::from(false, "finish homework"));
    mylist.add(Task::from(true, "do the dishes"));

    print_list(&mylist);
}

fn print_list(list: &ToDoList) {
    for item in list.iter() {
        println!("{}", item);
    }
}
