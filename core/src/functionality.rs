use relationships::Relationship;
use im::Vector;

use action::action_list_manipulation::ActionListManipulation;
use action::Action;
use relationships::RelationshipListManagement;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use std::fmt::Debug;
use std::cmp::PartialEq;
use std::error::Error;

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
struct ClearHeadApp  {
    action_list: Vector<Action>,
    relationship_list: Vector<Relationship>,
}

impl ClearHeadApp {
    fn create_action(&self) -> ClearHeadApp  {
        let mut cloned_list = self.clone();

        let new_action_list = self.action_list.create_new();
        cloned_list.action_list = new_action_list;

        cloned_list
        }

    fn create_relationship(&self, variant: &str, participant_1: Uuid, participant_2: Uuid) -> Result<ClearHeadApp, Box<dyn Error>> {
        let mut cloned_list = self.clone();

        let new_relationship_list: Vector<Relationship> = self.relationship_list.add_new(variant, participant_1, participant_2)?;
        cloned_list.relationship_list = new_relationship_list;

        Ok(cloned_list)
        }
}

enum ItemType {
    Action,
    Relationship,
}


#[cfg(test)]
mod tests {
    use super::*;
    use im::Vector;

    #[test]
    fn default_app_creation() {
        let test_app: ClearHeadApp = Default::default();
        assert_eq!(test_app.action_list, Vector::new());
        assert_eq!(test_app.relationship_list, Vector::new());
    }

    #[test]
    fn create_action() {
        let test_app: ClearHeadApp = Default::default();
        let updated_app = test_app.create_action();

        assert_eq!(updated_app.action_list.len(), 1);
    }

}
