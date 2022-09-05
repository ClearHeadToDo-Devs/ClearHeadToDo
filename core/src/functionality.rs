use relationships::Relationship;
use relationships::RelationshipListManagement;

use action::Action;
use action::ActionListManipulation;
use action::ActionError;
use action::Priority;


use std::fmt::Debug;
use std::cmp::PartialEq;
use std::error::Error;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
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

        let new_relationship_list: Vector<Relationship> = self.relationship_list.add_new(variant, self.action_list[participant_1].id, self.action_list[participant_2].id)?;
        cloned_list.relationship_list = new_relationship_list;

        Ok(cloned_list)
        }
}

impl ActionListManipulation for ClearHeadApp{
    fn append_default(&self) -> Self {
        let mut new_list = self.clone();

        new_list.action_list.push_back(Action::default());

        return new_list;
    }

    fn remove_action(&self, index: usize) -> Result<ClearHeadApp, Box<dyn Error>> {
        let mut new_list = self.clone();

        new_list.action_list.remove(index);

        return Ok(new_list);
    }

    fn rename(
        &self,
        index: usize,
        new_name: String,
    ) -> Result<ClearHeadApp, Box<dyn Error>> {
        let mut cloned_app = self.clone();

        let updated_action = self.select_by_index(index)?.rename(&new_name);
        let updated_list = self.action_list.update(index, updated_action);

        cloned_app.action_list = updated_list;
        Ok(cloned_app)
    }

    fn toggle_completion_status(
        &self,
        index: usize,
    ) -> Result<ClearHeadApp, Box<dyn Error>> {
        let mut cloned_list = self.clone();
        let updated_action = self.select_by_index(index)?.toggle_completion_status();

        let updated_list = cloned_list.action_list.update(index, updated_action);

        cloned_list.action_list = updated_list;

        Ok(cloned_list)
    }

    fn change_priority(
        &self,
        index: usize,
        new_priority: String,
    ) -> Result<ClearHeadApp, Box<dyn Error>> {
        let mut cloned_list = self.clone();

        let updated_action = self.select_by_index(index)?
            .change_priority(&new_priority)?;
        let updated_list = self.action_list.update(index, updated_action);

        cloned_list.action_list = updated_list;

        Ok(cloned_list)
    }

    fn select_by_id(&self, id: Uuid) -> Result<Action, Box<dyn Error>> {
        let search_action_result = self.clone().action_list.into_iter()
            .find(|actions| actions.get_id() == id);

        match search_action_result {
            Some(action) => return Ok(action.clone().to_owned()),
            None => {
                return Err(ActionError::InvalidId(id).into())
            }
        }
    }

    fn select_by_index(&self, index: usize) -> Result<Action, Box<dyn Error>> {
        match self.action_list.iter().nth(index) {
            Some(action_ref) => return Ok(action_ref.clone()),
            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }

    fn get_action_id(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        Ok(self.select_by_index(index)?.get_id())
    }

    fn get_action_name(&self, index: usize) -> Result<String, Box<dyn Error>> {
        Ok(self.select_by_index(index)?.get_name())
    }

    fn get_action_priority(&self, index: usize) -> Result<Priority, Box<dyn Error>> {
        Ok(self.select_by_index(index)?.get_priority())
    }
    fn get_action_completion_status(&self, index: usize) -> Result<bool, Box<dyn Error>> {
        Ok(self.select_by_index(index)?.get_completion_status())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use action::Priority;
    use im::Vector;
    use uuid::Uuid;


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

        assert_eq!(updated_app.action_list.get(0).unwrap().get_name(), "new_name");
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

    }

    #[test]
    fn list_all_actions_with_relationships(){
        let test_app: ClearHeadApp = ClearHeadApp::default()
            .create_action().create_action().create_relationship("parental", 0, 1).unwrap();

        let all_actions = test_app.get_extended_list().unwrap();

        assert_eq!(all_actions, format!("Order,Name,Priority,Completed,Id\n0,Default Action,Optional,false,{}\n  - Parental: Directed,Default Action,Optional,false,{}\n1,Default Action,Optional,false,{}\n", 
            test_app.action_list[0].id, 
            test_app.action_list[1].id, 
            test_app.action_list[1].id));

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
