use crate::action::Action;
use im::Vector;
use serde::{Serialize, Deserialize};



pub struct ExtendedAction{
    pub action: Action,

    pub parent: Option<Action>,
    pub children: Vector<Action>,
    pub predecessors: Vector<Action>,
    pub successors: Vector<Action>,
    pub related_actions: Vector<Action>,
}

impl Default for ExtendedAction{
    fn default() -> Self {
        ExtendedAction{
            action: Action::default(),
            children: Vector::new(),
            parent: None,
            predecessors: Vector::new(),
            successors: Vector::new(),
            related_actions: Vector::new(),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    use crate::ActionManipulation;


    #[test]
    fn create_default_extended_action(){
        let test_action = ExtendedAction::default();

        assert_eq!(test_action.action.get_name(), "Default Action");
        assert_eq!(test_action.action.get_priority(), "Optional");
        assert_eq!(test_action.action.get_completion_status(), false);
        assert_eq!(test_action.action.get_id().is_nil(), false);
        assert_eq!(test_action.children, Vector::new());
        assert_eq!(test_action.parent, None);
        assert_eq!(test_action.predecessors, Vector::new());
        assert_eq!(test_action.successors, Vector::new());
        assert_eq!(test_action.related_actions, Vector::new());
    }
}
