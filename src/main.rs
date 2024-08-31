use todo_list::fillers::{todo, Thing};
use todo_list::{Task, ToDoList};

fn main() {
    todo("finish the rest of the code");

    //let task = Task::from(false, "something random");
    //println!("task is : {}", task.task);

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
