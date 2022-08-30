use action::action_list_manipulation::ActionListManipulation;
use relationships::RelationshipListManagement;
use serde::{Serialize, Deserialize};

use std::fmt::Debug;
use std::cmp::PartialEq;
use std::error::Error;

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
struct ClearHeadApp<A: ActionListManipulation + Clone, R: RelationshipListManagement + Clone>  {
    action_list: A,
    relationship_list: R,
}

impl <A: ActionListManipulation + Clone,R: RelationshipListManagement + Clone>ClearHeadApp<A, R> {
    fn create(&self) -> ClearHeadApp<A, R>  {
        let mut cloned_list = self.clone();

        let new_action_list = self.action_list.create_new();
        cloned_list.action_list = new_action_list;

        cloned_list
        }
}

enum ItemType {
    Action,
    Relationship,
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Relationship;
    use action::Action;
    use im::Vector;

    #[test]
    fn default_app_creation() {
        let test_app: ClearHeadApp<Vector<Action>, Vector<Relationship>> = Default::default();
        assert_eq!(test_app.action_list, Vector::new());
        assert_eq!(test_app.relationship_list, Vector::new());
    }

    #[test]
    fn create_action() {
        let test_app: ClearHeadApp<Vector<Action>, Vector<Relationship>> = Default::default();
        let updated_app = test_app.create();

        assert_eq!(updated_app.action_list.len(), 1);
    }

}
