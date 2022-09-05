use action::Priority;
use relationships::Relationship;
use relationships::RelationshipListManagement;

use action::Action;
use action::ActionListManipulation;
use relationships::item::RelationshipVariant;


use std::fmt::Debug;
use std::cmp::PartialEq;
use std::error::Error;
use tabled::Table;

use serde::{Serialize, Deserialize};
use im::Vector;
use uuid::Uuid;

use crate::ClearHeadApp;

impl ActionListManipulation for ClearHeadApp {
    fn append_default(&self) -> Self {
        let mut new_app = self.clone();

        new_app.action_list = new_app.action_list.append_default();

        new_app
    }

    fn change_priority(
            &self,
            index: usize,
            new_priority: String,
        ) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized {
        unimplemented!();
    }

    fn rename(&self, index: usize, new_name: String) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized {
        let mut updated_list = self.clone();

        updated_list.action_list = updated_list.action_list.rename(index, new_name)?;

        Ok(updated_list)
    }

    fn toggle_completion_status(&self, index: usize) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized {
        let mut updated_list = self.clone();

        updated_list.action_list = updated_list.action_list.toggle_completion_status(index)?;
        Ok(updated_list)
    }

    fn select_by_id(&self, id: Uuid) -> Result<Action, Box<dyn Error>> {
        self.action_list.select_by_id(id)
    }

    fn select_by_index(&self, index: usize) -> Result<Action, Box<dyn Error>> {
        self.action_list.select_by_index(index)
    }

    fn get_action_name(&self, index: usize) -> Result<String, Box<dyn Error>> {
        self.action_list.get_action_name(index)
    }

    fn get_action_priority(&self, index: usize) -> Result<Priority, Box<dyn Error>> {
        self.action_list.get_action_priority(index)
    }

    fn get_action_completion_status(&self, index: usize) -> Result<bool, Box<dyn Error>> {
        self.action_list.get_action_completion_status(index)
    }

    fn get_action_id(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        self.action_list.get_action_id(index)
    }

    fn remove_action(&self, index: usize) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized {
        let mut updated_app = self.clone();

        updated_app.action_list = self.action_list.remove_action(index)?;

        Ok(updated_app)
    }

}

#[cfg(test)]
mod tests{
    use crate::{ClearHeadApp, functionality::tests::{create_app_with_single_action, failed_action_index_error, get_first_action}};

    use super::*;

    #[test]
    fn append_default_action(){
        let test_app = ClearHeadApp::default();

        let updated_app = test_app.create_action();

        assert_eq!(updated_app.action_list.len(), 1);

    }

    #[test]
    fn get_action_name(){
        let test_app = create_app_with_single_action();

        let action_name = test_app.get_action_name(0);

        assert_eq!(action_name.unwrap(), "Default Action");
    }

    #[test]
    fn failed_get_action_name(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.get_action_name(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn get_action_priority(){
        let test_app = create_app_with_single_action();

        let action_priority = test_app.get_action_priority(0);

        assert_eq!(action_priority.unwrap(), Priority::default());
    }

    #[test]
    fn failed_get_action_priority(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.get_action_priority(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn get_action_completion_status(){
        let test_app = create_app_with_single_action();

        let action_completion_status = test_app.get_action_completion_status(0);

        assert_eq!(action_completion_status.unwrap(), false);
    }

    #[test]
    fn failed_get_action_completion_status(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.get_action_completion_status(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn get_action_id(){
        let test_app = create_app_with_single_action();

        let action_id = test_app.get_action_id(0).unwrap();

        assert_eq!(action_id, get_first_action(&test_app).get_id());
    }

    #[test]
    fn failed_get_action_id(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.get_action_id(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn get_action_by_index(){
        let test_app = create_app_with_single_action();

        let action = test_app.select_by_index(0).unwrap();

        assert_eq!(action, get_first_action(&test_app));
    }

    #[test]
    fn failed_get_action_by_index(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.select_by_index(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn get_action_by_id(){
        let test_app = create_app_with_single_action();

        let action = test_app.select_by_id(test_app.action_list[0].get_id()).unwrap();

        assert_eq!(action, get_first_action(&test_app));
    }

    #[test]
    fn failed_get_action_by_id(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.select_by_id(Uuid::nil()).unwrap_err();

        let expected_error = format!("No Action with Id {}", Uuid::nil());
        assert_eq!(index_error.to_string(), expected_error);
    }

    #[test]
    fn remove_action(){
        let test_app = create_app_with_single_action();

        let updated_app = test_app.remove_action(0).unwrap();

        assert_eq!(updated_app.action_list.len(), 0);
    }

    #[test]
    fn failed_remove_action(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.remove_action(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn change_action_name(){
        let test_app = create_app_with_single_action();

        let updated_app = test_app.rename(0, "New Name".to_string()).unwrap();

        assert_eq!(get_first_action(&updated_app).get_name(), "New Name");
    }

    #[test]
    fn failed_change_action_name(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.rename(0, "New Name".to_string()).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn toggle_action_completion_status(){
        let test_app = create_app_with_single_action();

        let updated_app = test_app.toggle_completion_status(0).unwrap();

        assert_eq!(get_first_action(&updated_app).get_completion_status(), true);
    }
}
