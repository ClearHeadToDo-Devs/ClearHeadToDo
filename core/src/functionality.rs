use action::action_list_manipulation::ActionListManipulation;
use relationships::RelationshipListManagement;

use im::Vector;
use std::fmt::Debug;
use std::clone::Clone;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
struct ClearHeadApp<A: ActionListManipulation + Clone, R: RelationshipListManagement + Clone>  {
    action_list: Vector<A>,
    relationship_list: Vector<R>,
}

impl <A: ActionListManipulation + Clone,R: RelationshipListManagement + Clone>Default for ClearHeadApp<A, R> {
    fn default() -> Self {
        ClearHeadApp {
            action_list: Vector::new(),
            relationship_list: Vector::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Relationship;
    use action::Action;

    #[test]
    fn app_creation() {
        let test_app: ClearHeadApp<Vector<Action>, Vector<Relationship>> = ClearHeadApp {
            action_list: Vector::new(),
            relationship_list: Vector::new(),
        };
        assert_eq!(test_app.action_list, Vector::new());
        assert_eq!(test_app.relationship_list, Vector::new());
    }

    #[test]
    fn default_app_creation() {
        let test_app: ClearHeadApp<Vector<Action>, Vector<Relationship>> = Default::default();
        assert_eq!(test_app.action_list, Vector::new());
        assert_eq!(test_app.relationship_list, Vector::new());
    }

}
