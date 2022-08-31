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
pub struct ClearHeadApp  {
    pub action_list: Vector<Action>,
    pub relationship_list: Vector<Relationship>,
}

impl ClearHeadApp {
    pub fn create_action(&self) -> ClearHeadApp  {
        let mut cloned_list = self.clone();

        let new_action_list = self.action_list.create_new();
        cloned_list.action_list = new_action_list;

        cloned_list
        }
    pub fn rename_action(&self, index: usize, new_name: String) -> Result<ClearHeadApp, Box<dyn Error>> {
        let mut cloned_list = self.clone();

        let new_action_list = cloned_list.action_list.rename(index, new_name)?;
        cloned_list.action_list = new_action_list;

        Ok(cloned_list)
        }

    pub fn toggle_action_completion_status(&self, index: usize) -> Result<ClearHeadApp, Box<dyn Error>> {
        let mut cloned_list = self.clone();

        let new_action_list = cloned_list.action_list.toggle_completion_status(index)?;
        cloned_list.action_list = new_action_list;

        Ok(cloned_list)
        }

    pub fn change_action_priority(&self, index: usize, new_priority: String) -> Result<ClearHeadApp, Box<dyn Error>> {
        let mut cloned_list = self.clone();

        let new_action_list = cloned_list.action_list.change_priority(index, new_priority)?;
        cloned_list.action_list = new_action_list;

        Ok(cloned_list)
        }

    pub fn remove_action(&self, index: usize) -> Result<ClearHeadApp, Box<dyn Error>> {
        let mut cloned_list = self.clone();

        let new_action_list = cloned_list.action_list.remove(index)?;
        cloned_list.action_list = new_action_list;

        Ok(cloned_list)
        }

    pub fn get_list(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.action_list.get_list()?)
    }

    pub fn create_relationship(&self, variant: &str, participant_1: usize, participant_2: usize) -> Result<ClearHeadApp, Box<dyn Error>> {
        let mut cloned_list = self.clone();

        let new_relationship_list: Vector<Relationship> = self.relationship_list.add_new(variant, self.action_list[participant_1].id, self.action_list[participant_2].id)?;
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
    use action::Priority;
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

    #[test]
    fn rename_action(){
        let test_app: ClearHeadApp = Default::default();
        let default_action_app = test_app.create_action();

        let updated_app = default_action_app.rename_action(0, "new_name".to_string()).unwrap();

        assert_eq!(updated_app.action_list.get(0).unwrap().name, "new_name");
    }

    #[test]
    fn toggle_action_completion_status(){
        let test_app: ClearHeadApp = Default::default();
        let default_action_app = test_app.create_action();

        let updated_app = default_action_app.toggle_action_completion_status(0).unwrap();

        assert_eq!(updated_app.action_list.get(0).unwrap().completed, true);
    }

    #[test]
    fn remove_action(){
        let test_app: ClearHeadApp = Default::default();
        let default_action_app = test_app.create_action();

        let updated_app = default_action_app.remove_action(0).unwrap();

        assert_eq!(updated_app.action_list.len(), 0);
    }

    #[test]
    fn change_action_priority(){
        let test_app: ClearHeadApp = Default::default();
        let default_action_app = test_app.create_action();

        let updated_app = default_action_app.change_action_priority(0, 1.to_string()).unwrap();

        assert_eq!(updated_app.action_list.get(0).unwrap().priority, Priority::Critical);
    }

    #[test]
    fn list_all_actions(){
        let test_app: ClearHeadApp = Default::default();
        let mut default_action_app = test_app.create_action();
        default_action_app.action_list[0].id = Uuid::nil();


        let all_actions = default_action_app.get_list().unwrap();

        assert_eq!(all_actions, format!("name,priority,completed,ID\nDefault Action,Optional,false,{}\n", Uuid::nil()));
    }

    #[test]
    fn create_relationship(){
        let test_app: ClearHeadApp = ClearHeadApp::default().create_action().create_action();
        let updated_app = test_app.create_relationship("related", 0,1).unwrap();

        assert_eq!(updated_app.relationship_list.len(), 1);
    }

    #[test]
    #[should_panic]
    fn failed_non_existant_action_relationship(){
        let test_app: ClearHeadApp = Default::default();

        test_app.create_relationship("related", 0, 1).unwrap();
    }
}
