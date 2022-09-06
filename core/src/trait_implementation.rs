use action::Priority;
use relationships::Relationship;
use relationships::RelationshipListManagement;

use action::Action;
use action::ActionListManipulation;
use relationships::item::RelationshipVariant;


use std::error::Error;

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
        let mut new_app = self.clone();

        new_app.action_list = new_app.action_list.change_priority(index, new_priority)?;

        Ok(new_app)
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

impl RelationshipListManagement for ClearHeadApp {
    type L = ClearHeadApp;
    fn add_new(
            &self,
            target_variant: &str,
            participant_1: Uuid,
            participant_2: Uuid,
        ) -> Result<Self::L, Box<dyn Error>> {
        let mut updated_app = self.clone();

        let updated_relationship_list = self.relationship_list.add_new(
            target_variant,
            participant_1,
            participant_2,
        )?;

        updated_app.relationship_list = updated_relationship_list;

        Ok(updated_app)
    }

    fn add_related(&self, participant_1: Uuid, participant_2: Uuid) -> ClearHeadApp {
        unimplemented!()
    }

    fn add_parental(&self, participant_1: Uuid, participant_2: Uuid) -> ClearHeadApp {
        unimplemented!()
    }

    fn add_sequential(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        unimplemented!()
    }

    fn select_by_id(&self, id: Uuid) -> Result<Relationship, String> {
        unimplemented!()
    }

    fn select_by_index(&self, index: usize) -> Result<Relationship, Box<dyn Error>> {
        unimplemented!()
    }

    fn get_variant(&self, index: usize) -> Result<RelationshipVariant, Box<dyn Error>> {
        unimplemented!()
    }

    fn get_id(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        unimplemented!()
    }

    fn get_participant_1(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        unimplemented!()
    }

    fn get_participant_2(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        unimplemented!()
    }

    fn remove_at_index(&self, index: usize) -> Result<Self::L, Box<dyn Error>> {
        unimplemented!()
    }

    fn remove_with_id(&self, id: Uuid) -> Result<Self::L, Box<dyn Error>> {
        unimplemented!()
    }

    fn update_participant_1(&self, index: usize, new_id: Uuid) -> Result<Self::L, Box<dyn Error>> {
        unimplemented!()
    }

    fn update_participant_2(&self, index: usize, new_id: Uuid) -> Result<Self::L, Box<dyn Error>> {
        unimplemented!()
    }

    fn change_variant(&self, index: usize, variant: &str) -> Result<Self::L, Box<dyn Error>> {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests{
    use crate::{ClearHeadApp, functionality::tests::{create_app_with_single_action, failed_action_index_error, get_first_action, create_minimal_related_app, create_app_with_two_actions}};

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

        let action = action::ActionListManipulation::select_by_index(&test_app, 0).unwrap();

        assert_eq!(action, get_first_action(&test_app));
    }

    #[test]
    fn failed_get_action_by_index(){
        let empty_app = ClearHeadApp::default();

        let index_error = action::ActionListManipulation::select_by_index(&empty_app, 0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn get_action_by_id(){
        let test_app = create_app_with_single_action();

        let action = action::ActionListManipulation::select_by_id(&test_app, test_app.action_list[0].get_id()).unwrap();

        assert_eq!(action, get_first_action(&test_app));
    }

    #[test]
    fn failed_get_action_by_id(){
        let empty_app = ClearHeadApp::default();

        let index_error = action::ActionListManipulation::select_by_id(&empty_app, Uuid::nil()).unwrap_err();

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

    #[test]
    fn failed_toggle_action_completion_status(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.toggle_completion_status(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn change_action_priority(){
        let test_app = create_app_with_single_action();

        let updated_app = test_app.change_priority(0, "high".to_string()).unwrap();

        assert_eq!(get_first_action(&updated_app).get_priority(), Priority::High);
    }

    #[test]
    fn failed_change_action_priority(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.change_priority(0, "high".to_string()).unwrap_err();

        assert_eq!(index_error.to_string(), failed_action_index_error(0));
    }

    #[test]
    fn failed_change_priority_invalid_priority(){
        let test_app = create_app_with_single_action();

        let index_error = test_app.change_priority(0, "invalid".to_string()).unwrap_err();

        assert_eq!(index_error.to_string(), "invalid is an Invalid Priority Option");
    }

    #[test]
    fn create_relationship() {
        let test_app = ClearHeadApp::default();

        let updated_app = test_app.add_new("related", Uuid::nil(), Uuid::nil()).unwrap();

        assert_eq!(updated_app.relationship_list.len(), 1);
    }

    #[test]
    fn failed_create_relationship(){
        let test_app = ClearHeadApp::default();

        let bad_variant_error = test_app.add_new("invalid", Uuid::nil(), Uuid::nil()).unwrap_err();

        assert_eq!(bad_variant_error.to_string(), "invalid relationship variant");
    }

    #[test]
    fn create_related(){
        let test_app = ClearHeadApp::default();

        let updated_app = test_app.add_new("related", Uuid::nil(), Uuid::nil()).unwrap();

        assert_eq!(updated_app.relationship_list.len(), 1);
    }

    #[test]
    fn create_sequential(){
        let test_app = ClearHeadApp::default();

        let updated_app = test_app.add_new("sequential", Uuid::nil(), Uuid::nil()).unwrap();

        assert_eq!(updated_app.relationship_list.len(), 1);
    }

    #[test]
    fn create_parental(){
        let test_app = ClearHeadApp::default();

        let updated_app = test_app.add_new("parental", Uuid::nil(), Uuid::nil()).unwrap();

        assert_eq!(updated_app.relationship_list.len(), 1);
    }
}
