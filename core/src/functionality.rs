use relationships::Relationship;
use relationships::RelationshipListManagement;

use action::Action;
use action::ActionListManipulation;


use std::fmt::Debug;
use std::cmp::PartialEq;
use std::error::Error;
use tabled::Table;

use serde::{Serialize, Deserialize};
use im::Vector;


#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct ClearHeadApp  {
    pub action_list: Vector<Action>,
    pub relationship_list: Vector<Relationship>,
}

impl ClearHeadApp {

    pub fn get_list(&self) -> Table {
        Table::new(&self.action_list)
    }

    pub fn get_extended_list(&self) -> Result<String, Box<dyn Error>> {
        let mut extended_list = String::new();
        let mut index = 0;

        extended_list.push_str("Order,Name,Priority,Completed,Id\n");
        for action in &self.action_list {

            extended_list.push_str(&format!("{},{}\n",index, action.to_string()));
            index += 1;

            if self.id_is_present_in_participant_1_list(action.get_id()) {
                for relationship in &self.get_participant_1_list_for_id(action.get_id())? {
                        extended_list.push_str(&format!(
                            "  - {},{}\n",relationship.get_variant(), 
                            &self.action_list.select_action_by_id(relationship.get_participant_2())?));
                    }
                }
            }
        Ok(extended_list)
    }

    pub fn create_action_relationship(&self, variant_str: &str, participant_1_index: usize, participant_2_index: usize) -> Result<ClearHeadApp, Box<dyn Error>> {
        let participant_1_id = self.get_action_id(participant_1_index)?;
        let participant_2_id = self.get_action_id(participant_2_index)?;

        let updated_app = self.append_new_relationship(variant_str, participant_1_id, participant_2_id)?;

        Ok(updated_app)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use im::Vector;


    pub fn create_app_with_single_action() -> ClearHeadApp {
        let app = ClearHeadApp::default().append_default_action();

        app
    }

    pub fn create_app_with_two_actions() -> ClearHeadApp {
        let app = ClearHeadApp::default().append_default_action().append_default_action();

        app
    }

    pub fn get_first_action(app: &ClearHeadApp) -> Action {
        app.action_list[0].clone()
    }

    pub fn failed_action_index_error(index: usize) -> String {
        format!("No Action at Index {}",index.to_string())
    }

    pub fn failed_relationship_index_error() -> String {
        "Unable to find Relationship at given Index".to_string()
    }

    pub fn create_minimal_related_app(variant_str: &str) -> ClearHeadApp {
        let app = ClearHeadApp::default()
            .append_default_action()
            .append_default_action()
            .create_action_relationship(variant_str, 0, 1).unwrap();

        app
    }


    #[test]
    fn default_app_creation() {
        let test_app: ClearHeadApp = Default::default();

        assert_eq!(test_app.action_list, Vector::new());
        assert_eq!(test_app.relationship_list, Vector::new());
    }

    #[test]
    fn list_all_actions(){
        let test_app = create_app_with_single_action();

        let action_list_string = test_app.get_list();

        let expected_string = format!(
"+----------------+----------+-----------+--------------------------------------+
| name           | priority | completed | id                                   |
+----------------+----------+-----------+--------------------------------------+
| Default Action | Optional | false     | {} |
+----------------+----------+-----------+--------------------------------------+"
            ,test_app.action_list[0].get_id().hyphenated());

        assert_eq!(action_list_string.to_string(), expected_string);

    }

    #[test]
    fn list_all_actions_with_relationships(){
        let test_app = create_minimal_related_app("parental");

        let all_actions = test_app.get_extended_list().unwrap();


        let expected_string = format!(
"Order,Name,Priority,Completed,Id
0,Default Action,Optional,false,{}
  - Parental: Directed,Default Action,Optional,false,{}
1,Default Action,Optional,false,{}\n", 
            test_app.action_list[0].get_id(),
            test_app.action_list[1].get_id(),
            test_app.action_list[1].get_id());

        assert_eq!(all_actions, expected_string);

    }
}
