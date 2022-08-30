use action::action_list_manipulation::ActionListManipulation;
use relationships::RelationshipListManagement;

use std::fmt::Debug;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq, Default)]
struct ClearHeadApp<A: ActionListManipulation, R: RelationshipListManagement>  {
    action_list: A,
    relationship_list: R,
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
}
