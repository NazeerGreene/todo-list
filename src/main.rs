use todo_list::fillers::{todo, Thing};
use todo_list::{Task, ToDoList};

fn main() {
    todo("finish the rest of the code");

    //let task = Task::from(false, "something random");
    //println!("task is : {}", task.task);

    // let mut mylist = ToDoList::new();

    // mylist.add(Task::from(false, "finish homework"));
    // mylist.add(Task::from(true, "do the dishes"));

    // print_list(&mylist);
    // println!("saving to file...");
    // mylist.save_to("todolist.txt");
    // println!("saved to file!");

    let todolist = ToDoList::load_from("todolist.txt").unwrap_or(ToDoList::new());
    println!("size of list is {}", todolist.size());
    print_list(&todolist);
}

fn print_list(list: &ToDoList) {
    for item in list.iter() {
        println!("{}", item);
    }
}
