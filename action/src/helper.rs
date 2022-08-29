use crate::item::Action;
use uuid::Uuid;

pub fn add_nil_action(action_list: im::Vector<Action>) -> im::Vector<Action> {
    let mut new_list = action_list.clone();
    let new_action: Action = Action {
        id: Uuid::nil(),
        ..Default::default()
    };
    new_list.push_back(new_action);
    return new_list;
}
