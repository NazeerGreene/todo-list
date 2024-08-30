use todo_list::fillers::{todo, Thing};
use todo_list::to_do::{Task, ToDoList};

fn main() {
    let mut first = Task::from(false, "finish homework");

    let mut mylist = ToDoList::new();
    mylist.add(first);

    mylist.add(Task::from(true, "do the dishes"));

    mylist.print();

    mylist.toggle_task(1);

    mylist.print();

    mylist.toggle_task(0);

    mylist.print();
}
