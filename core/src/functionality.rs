use relationships::Relationship;
use relationships::RelationshipListManagement;

use action::Action;
use action::ActionListManipulation;


use std::fmt::Debug;
use std::cmp::PartialEq;
use std::error::Error;

use serde::{Serialize, Deserialize};
use im::Vector;


#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct ClearHeadApp  {
    pub action_list: Vector<Action>,
    pub relationship_list: Vector<Relationship>,
}

impl ClearHeadApp {
    pub fn create_action(&self) -> ClearHeadApp  {
        let mut cloned_list = self.clone();

        let new_action_list = self.action_list.append_default();
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

        let new_action_list = cloned_list.action_list.remove_action(index)?;
        cloned_list.action_list = new_action_list;

        Ok(cloned_list)
        }

    pub fn get_list(&self) -> String {
            format!("{:?}",self.action_list)
    }

    pub fn get_extended_list(&self) -> Result<String, Box<dyn Error>> {
        let mut extended_list = String::new();
        let mut index = 0;

        extended_list.push_str("Order,Name,Priority,Completed,Id\n");
        for action in &self.action_list {
            extended_list.push_str(&format!("{},{}\n",index, action.to_string()));
            index += 1;
            if self.relationship_list.iter().find(|relationship| relationship.get_participant_1() == action.id).is_some() {
                for relationship in self.relationship_list.iter().filter(
                    |relationship| relationship.get_participant_1() == action.id) {
                    extended_list.push_str(&format!(
                        "  - {},{}\n",relationship.get_variant(), 
                        &self.action_list.select_by_id(relationship.get_participant_2())?));
                }
            }
        }
        Ok(extended_list)
    }
    

    pub fn create_relationship(&self, variant: &str, participant_1: usize, participant_2: usize) -> Result<ClearHeadApp, Box<dyn Error>> {
        let mut cloned_list = self.clone();
        let participant_1_id = self.action_list.select_by_index(participant_1)?.get_id();
        let participant_2_id = self.action_list.select_by_index(participant_2)?.get_id();

        let updated_relationship_list: Vector<Relationship> = 
        self.relationship_list.add_new(
            variant, participant_1_id, participant_2_id)?;

        cloned_list.relationship_list = updated_relationship_list;

        Ok(cloned_list)
        }

    pub fn change_relationship_variant(&self, index: usize, new_variant: &str) -> Result<ClearHeadApp, Box<dyn Error>> {
        let mut cloned_list = self.clone();

        let updated_relationship_list: Vector<Relationship> = 
        self.relationship_list.change_variant(index, new_variant)?;

        cloned_list.relationship_list = updated_relationship_list;

        Ok(cloned_list)
        }
}


#[cfg(test)]
mod tests {
    use super::*;
    use im::Vector;

    use action::Priority;

    use relationships::item::RelationshipVariant;

    pub fn create_app_with_single_action() -> ClearHeadApp {
        let app = ClearHeadApp::default().create_action();

        app
    }

    pub fn create_app_with_two_actions() -> ClearHeadApp {
        let app = ClearHeadApp::default().create_action().create_action();

        app
    }

    pub fn get_first_action(app: &ClearHeadApp) -> Action {
        app.action_list[0].clone()
    }

    pub fn failed_index_error(index: usize) -> String {
        format!("No Action at Index {}",index.to_string())
    }

    pub fn create_minimal_related_app(variant_str: &str) -> ClearHeadApp {
        let app = ClearHeadApp::default()
            .create_action()
            .create_action()
            .create_relationship(variant_str, 0 , 1).unwrap();

        app
    }


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
    fn successful_rename_action(){
        let test_app = create_app_with_single_action();

        let updated_app = test_app.rename_action(0, "new_name".to_string()).unwrap();

        assert_eq!(get_first_action(&updated_app).get_name(), "new_name");
    }

    #[test]
    fn failed_rename_action(){
        let test_app = ClearHeadApp::default();

        let failed_update = test_app.rename_action(0, "new_name".to_string()).unwrap_err();

        assert_eq!(failed_update.to_string(), failed_index_error(0));
    }

    #[test]
    fn toggle_action_completion_status(){
        let test_app = create_app_with_single_action();

        let updated_app = test_app.toggle_action_completion_status(0).unwrap();

        assert_eq!(get_first_action(&updated_app).get_completion_status(), true);
    }

    #[test]
    fn failed_toggle_action_completion_status(){
        let test_app = ClearHeadApp::default();

        let failed_update = test_app.toggle_action_completion_status(0).unwrap_err();

        assert_eq!(failed_update.to_string(), failed_index_error(0));
    }

    #[test]
    fn successful_remove_action(){
        let test_app = create_app_with_single_action();

        let updated_app = test_app.remove_action(0).unwrap();

        assert_eq!(updated_app.action_list.len(), 0);
    }

    #[test]
    fn failed_remove_action(){
        let test_app = ClearHeadApp::default();

        let index_error = test_app.remove_action(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_index_error(0));
    }

    #[test]
    fn change_action_priority(){
        let test_app = create_app_with_single_action();

        let updated_app = test_app.change_action_priority(0, 1.to_string()).unwrap();

        assert_eq!(get_first_action(&updated_app).get_priority(), Priority::Critical);
    }

    #[test]
    fn list_all_actions(){
        let test_app = create_app_with_single_action();

        let action_list_string = test_app.get_list();
        let expected_string = format!(
            "[Action {{ name: \"Default Action\", priority: Optional, completed: false, id: {} }}]",
            test_app.action_list[0].get_id());

        assert_eq!(action_list_string, expected_string);

    }

    #[test]
    fn list_all_actions_with_relationships(){
        let test_app: ClearHeadApp = ClearHeadApp::default()
            .create_action().create_action().create_relationship("parental", 0, 1).unwrap();

        let all_actions = test_app.get_extended_list().unwrap();

        assert_eq!(all_actions, format!(
"Order,Name,Priority,Completed,Id
0,Default Action,Optional,false,{}
  - Parental: Directed,Default Action,Optional,false,{}
1,Default Action,Optional,false,{}\n", 
            test_app.action_list[0].get_id(),
            test_app.action_list[1].get_id(),
            test_app.action_list[1].get_id()));

    }

    #[test]
    fn create_relationship(){
        let test_app: ClearHeadApp = create_app_with_two_actions();

        let updated_app = test_app.create_relationship("related", 0,1).unwrap();

        assert_eq!(updated_app.relationship_list.len(), 1);
    }

    #[test]
    fn create_sequential_relationship(){
        let test_app = create_app_with_two_actions();

        let updated_app = test_app.create_relationship("sequential", 0,1).unwrap();

        assert_eq!(updated_app.relationship_list.len(), 1);
    }

    #[test]
    fn create_parental_relationship(){
        let test_app = create_app_with_two_actions();

        let updated_app = test_app.create_relationship("parental", 0,1).unwrap();

        assert_eq!(updated_app.relationship_list.len(), 1);
    }

    #[test]
    fn failed_non_existant_action_relationship(){
        let test_app: ClearHeadApp = Default::default();

        let invalid_index_error = test_app.create_relationship("related", 0, 1).unwrap_err();

        assert_eq!(invalid_index_error.to_string(), failed_index_error(0));
    }

    #[test]
    fn failed_invalid_relationship_type(){
        let test_app = create_app_with_two_actions();

        let invalid_relationship_error = test_app.create_relationship("invalid", 0, 1).unwrap_err();

        assert_eq!(invalid_relationship_error.to_string(), "invalid relationship variant");
    }

    #[test]
    fn change_relationship_variant_by_index(){
        let test_app = create_minimal_related_app("related");

        let updated_app = test_app.change_relationship_variant(0, "parental").unwrap();

        assert_eq!(updated_app.relationship_list[0].get_variant(), RelationshipVariant::create_parental());
    }

    #[test]
    fn change_relationship_variant_to_sequential(){
        let test_app = create_minimal_related_app("related");

        let updated_app = test_app.change_relationship_variant(0, "sequential").unwrap();

        assert_eq!(updated_app.relationship_list[0].get_variant(), RelationshipVariant::create_sequential());
    }

    #[test]
    fn change_relationship_variant_to_related(){
        let test_app = create_minimal_related_app("parental");

        let updated_app = test_app.change_relationship_variant(0, "related").unwrap();

        assert_eq!(updated_app.relationship_list[0].get_variant(), RelationshipVariant::create_related());
    }
}
