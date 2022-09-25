use crate::ClearHeadApp;

use crate::action::Action;

use std::error::Error;
use uuid::Uuid;

pub trait ActionFunctionality {
    type Action: ActionFunctionality;
    fn get_name(&self) -> String;
    fn get_priority(&self) -> String;
    fn get_completion_status(&self) -> bool;
    fn get_id(&self) -> Uuid;

    fn rename(&self, new_name: &str) -> Self::Action;
    fn change_priority(&self, new_priority: &str) -> Result<Self::Action, Box<dyn Error>> where Self: Sized;
    fn toggle_completion_status(&self) -> Self::Action;
}

pub trait ActionListManipulation {
    type Item;
    fn append_default_action(&self) -> Self;

    fn rename_action(&self, index: usize, new_name: String) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn toggle_action_completion_status(&self, index: usize) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn change_action_priority(
        &self,
        index: usize,
        new_priority: String,
    ) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    fn select_action_by_id(&self, id: Uuid) -> Result<Self::Item, Box<dyn Error>>;
    fn select_action_by_index(&self, index: usize) -> Result<Self::Item, Box<dyn Error>>;

    fn get_action_name(&self, index: usize) -> Result<String, Box<dyn Error>>;
    fn get_action_priority(&self, index: usize) -> Result<String, Box<dyn Error>>;
    fn get_action_completion_status(&self, index: usize) -> Result<bool, Box<dyn Error>>;
    fn get_action_id(&self, index: usize) -> Result<Uuid, Box<dyn Error>>;

    fn remove_action(&self, index: usize) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

impl ClearHeadApp {
    pub fn append_default_action(&self) -> Self {
        let mut new_app = self.clone();

        new_app.action_list = new_app.action_list.append_default_action();

        new_app
    }

    pub fn change_action_priority(
            &self,
            index: usize,
            new_priority: String,
        ) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized {
        let mut new_app = self.clone();

        new_app.action_list = new_app.action_list.change_action_priority(index, new_priority)?;

        Ok(new_app)
    }

    pub fn rename_action(&self, index: usize, new_name: String) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized {
        let mut updated_list = self.clone();

        updated_list.action_list = updated_list.action_list.rename_action(index, new_name)?;

        Ok(updated_list)
    }

    pub fn toggle_action_completion_status(&self, index: usize) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized {
        let mut updated_list = self.clone();

        updated_list.action_list = updated_list.action_list.toggle_action_completion_status(index)?;
        Ok(updated_list)
    }

    pub fn select_action_by_id(&self, id: Uuid) -> Result<Action, Box<dyn Error>> {
        self.action_list.select_action_by_id(id)
    }

    pub fn select_action_by_index(&self, index: usize) -> Result<Action, Box<dyn Error>> {
        self.action_list.select_action_by_index(index)
    }

    pub fn get_action_name(&self, index: usize) -> Result<String, Box<dyn Error>> {
        self.action_list.get_action_name(index)
    }

    pub fn get_action_priority(&self, index: usize) -> Result<String, Box<dyn Error>> {
        self.action_list.get_action_priority(index)
    }

    pub fn get_action_completion_status(&self, index: usize) -> Result<bool, Box<dyn Error>> {
        self.action_list.get_action_completion_status(index)
    }

    pub fn get_action_id(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        self.action_list.get_action_id(index)
    }

    pub fn remove_action(&self, index: usize) -> Result<Self, Box<dyn Error>>
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

        let updated_app = test_app.append_default_action();

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

        assert_eq!(action_priority.unwrap(), "Optional");
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

        let action = test_app.select_action_by_index(0).unwrap();

        assert_eq!(action, get_first_action(&test_app));
    }

    #[test]
    fn failed_get_action_by_index(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.select_action_by_index(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn get_action_by_id(){
        let test_app = create_app_with_single_action();

        let action = test_app.select_action_by_id(test_app.action_list[0].get_id()).unwrap();

        assert_eq!(action, get_first_action(&test_app));
    }

    #[test]
    fn failed_get_action_by_id(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.select_action_by_id(Uuid::nil()).unwrap_err();

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

        let updated_app = test_app.rename_action(0, "New Name".to_string()).unwrap();

        assert_eq!(get_first_action(&updated_app).get_name(), "New Name");
    }

    #[test]
    fn failed_change_action_name(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.rename_action(0, "New Name".to_string()).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn toggle_action_completion_status(){
        let test_app = create_app_with_single_action();

        let updated_app = test_app.toggle_action_completion_status(0).unwrap();

        assert_eq!(get_first_action(&updated_app).get_completion_status(), true);
    }

    #[test]
    fn failed_toggle_action_completion_status(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.toggle_action_completion_status(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn change_action_priority(){
        let test_app = create_app_with_single_action();

        let updated_app = test_app.change_action_priority(0, "high".to_string()).unwrap();

        assert_eq!(get_first_action(&updated_app).get_priority(), "High");
    }

    #[test]
    fn failed_change_action_priority(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.change_action_priority(0, "high".to_string()).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn failed_change_priority_invalid_priority(){
        let test_app = create_app_with_single_action();

        let index_error = test_app.change_action_priority(0, "invalid".to_string()).unwrap_err();

        assert_eq!(index_error.to_string(), "invalid is an Invalid Priority Option");
    }
}

