use crate::item::Action;
use uuid::Uuid;

pub fn add_nil_action(task_list: im::Vector<Action>) -> im::Vector<Action> {
    let mut new_list = task_list.clone();
    let new_task: Action = Action {
        id: Uuid::nil(),
        ..Default::default()
    };
    new_list.push_back(new_task);
    return new_list;
}
