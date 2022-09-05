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
        unimplemented!();
    }

    fn toggle_completion_status(&self, index: usize) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized {
        unimplemented!();
    }

    fn select_by_id(&self, id: Uuid) -> Result<Action, Box<dyn Error>> {
        unimplemented!();
    }

    fn select_by_index(&self, index: usize) -> Result<Action, Box<dyn Error>> {
        unimplemented!();
    }

    fn get_action_name(&self, index: usize) -> Result<String, Box<dyn Error>> {
        self.action_list.get_action_name(index)
    }

    fn get_action_priority(&self, index: usize) -> Result<Priority, Box<dyn Error>> {
        self.action_list.get_action_priority(index)
    }

    fn get_action_completion_status(&self, index: usize) -> Result<bool, Box<dyn Error>> {
        unimplemented!();
    }

    fn get_action_id(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        unimplemented!();
    }

    fn remove_action(&self, index: usize) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized {
        unimplemented!();
    }

}

#[cfg(test)]
mod tests{
    use crate::{ClearHeadApp, functionality::tests::{create_app_with_single_action, failed_action_index_error}};

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
}
