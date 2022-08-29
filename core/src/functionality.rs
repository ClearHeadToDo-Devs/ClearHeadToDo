use std::error::Error;

use action::{action_list_manipulation::ActionListManipulation, ActionManipulation};
use relationships::{RelationshipListManagement, RelationshipManagement};

use im::Vector;
struct ClearHeadApp<A: ActionListManipulation, R: RelationshipListManagement>  {
    action_list: Vector<A>,
    relationship_list: Vector<R>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use action::Action;
    use relationships::Relationship;

    #[test]
    fn test_app_creation() {
        let test_app: ClearHeadApp<Vector<Action>, Vector<Relationship>> = ClearHeadApp {
            action_list: Vector::new(),
            relationship_list: Vector::new(),
        };
        assert_eq!(test_app.action_list, Vector::new());
        assert_eq!(test_app.relationship_list, Vector::new());
    }
}
