use super::task::Task;
use uuid::Uuid;

pub fn add_nil_task(task_list: im::Vector<Task>) -> im::Vector<Task> {
    let mut new_list = task_list.clone();
    let new_task: Task = Task {
        id: Uuid::nil(),
        ..Default::default()
    };
    new_list.push_back(new_task);
    return new_list;
}
