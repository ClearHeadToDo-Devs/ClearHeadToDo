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
        let mut cloned_app = self.clone();

        let updated_relationship_list = self.relationship_list.add_related(
            participant_1,
            participant_2,
        );

        cloned_app.relationship_list = updated_relationship_list;

        return cloned_app;
    }

    fn add_parental(&self, participant_1: Uuid, participant_2: Uuid) -> ClearHeadApp {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self.relationship_list.add_parental(
            participant_1,
            participant_2,
        );

        cloned_app.relationship_list = updated_relationship_list;

        return cloned_app;
    }

    fn add_sequential(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self.relationship_list.add_sequential(
            participant_1,
            participant_2,
        );

        cloned_app.relationship_list = updated_relationship_list;

        return cloned_app;
    }

    fn select_by_id(&self, id: Uuid) -> Result<Relationship, String> {
        Ok(self.relationship_list.select_by_id(id)?)
    }

    fn select_by_index(&self, index: usize) -> Result<Relationship, Box<dyn Error>> {
        Ok(self.relationship_list.select_by_index(index)?)
    }

    fn get_variant(&self, index: usize) -> Result<RelationshipVariant, Box<dyn Error>> {
        Ok(self.relationship_list.get_variant(index)?)
    }

    fn get_id(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        Ok(self.relationship_list.get_id(index)?)
    }

    fn get_participant_1(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        Ok(self.relationship_list.get_participant_1(index)?)
    }

    fn get_participant_2(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        Ok(self.relationship_list.get_participant_2(index)?)
    }

    fn remove_at_index(&self, index: usize) -> Result<Self::L, Box<dyn Error>> {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self.relationship_list.remove_at_index(index)?;
        cloned_app.relationship_list = updated_relationship_list;

        Ok(cloned_app)
    }

    fn remove_with_id(&self, id: Uuid) -> Result<Self::L, Box<dyn Error>> {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self.relationship_list.remove_with_id(id)?;
        cloned_app.relationship_list = updated_relationship_list;

        Ok(cloned_app)
    }

    fn update_participant_1(&self, index: usize, new_id: Uuid) -> Result<Self::L, Box<dyn Error>> {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self.relationship_list.update_participant_1(index, new_id)?;
        cloned_app.relationship_list = updated_relationship_list;

        Ok(cloned_app)
    }

    fn update_participant_2(&self, index: usize, new_id: Uuid) -> Result<Self::L, Box<dyn Error>> {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self.relationship_list.update_participant_2(index, new_id)?;
        cloned_app.relationship_list = updated_relationship_list;

        Ok(cloned_app)
    }

    fn change_variant(&self, index: usize, variant: &str) -> Result<Self::L, Box<dyn Error>> {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self.relationship_list.change_variant(index, variant)?;
        cloned_app.relationship_list = updated_relationship_list;

        Ok(cloned_app)
    }
}


#[cfg(test)]
mod tests{
    use crate::{ClearHeadApp, functionality::tests::{create_app_with_single_action, failed_action_index_error, get_first_action, create_minimal_related_app, create_app_with_two_actions, failed_relationship_index_error}};

    use super::*;

    pub fn create_app_with_single_relationship(variant_str: &str) -> ClearHeadApp {
        ClearHeadApp::default().add_new(variant_str, Uuid::nil(), Uuid::nil()).unwrap()
    }

    pub fn failed_relationship_id_search_error() -> String {
        "cannot find this id within the relationship list".to_string()
    }

    pub fn failed_relationship_variant_error() -> String {
        "invalid relationship variant".to_string()
    }

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

        assert_eq!(bad_variant_error.to_string(), failed_relationship_variant_error());
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

        assert_eq!(updated_app.get_variant(0).unwrap(), RelationshipVariant::create_parental());
    }

    #[test]
    fn create_related_direct(){
        let test_app = ClearHeadApp::default();

        let updated_app = test_app.add_related(Uuid::nil(), Uuid::nil());

        assert_eq!(updated_app.get_variant(0).unwrap(), RelationshipVariant::create_related());
    }

    #[test]
    fn create_sequential_direct(){
        let test_app = ClearHeadApp::default();

        let updated_app = test_app.add_sequential(Uuid::nil(), Uuid::nil());

        assert_eq!(updated_app.get_variant(0).unwrap(), RelationshipVariant::create_sequential());
    }

    #[test]
    fn create_parental_direct(){
        let test_app = ClearHeadApp::default();

        let updated_app = test_app.add_parental(Uuid::nil(), Uuid::nil());

        assert_eq!(updated_app.get_variant(0).unwrap(), RelationshipVariant::create_parental());
    }

    #[test]
    fn get_variant(){
        let test_app = create_app_with_single_relationship("related");

        let variant = test_app.get_variant(0).unwrap();

        assert_eq!(variant, RelationshipVariant::create_related());
    }

    #[test]
    fn failed_get_variant(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.get_variant(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn get_id(){
        let test_app = create_app_with_single_relationship("related");

        let id = test_app.get_id(0).unwrap();

        assert_eq!(id, test_app.relationship_list.get_id(0).unwrap());
    }

    #[test]
    fn failed_get_id(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.get_id(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn get_participant_1(){
        let test_app = create_app_with_single_relationship("related");

        let participant_1 = test_app.get_participant_1(0).unwrap();

        assert_eq!(participant_1, Uuid::nil());
    }

    #[test]
    fn failed_get_participant_1(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.get_participant_1(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn get_participant_2(){
        let test_app = create_app_with_single_relationship("related");

        let participant_2 = test_app.get_participant_2(0).unwrap();

        assert_eq!(participant_2, Uuid::nil());
    }

    #[test]
    fn failed_get_participant_2(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.get_participant_2(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn remove_relationship(){
        let test_app = create_app_with_single_relationship("related");

        let updated_app = test_app.remove_at_index(0).unwrap();

        assert_eq!(updated_app.relationship_list.len(), 0);
    }

    #[test]
    fn failed_remove_relationship(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.remove_at_index(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn remove_relationship_by_id(){
        let test_app = create_app_with_single_relationship("related");
        let target_id = test_app.get_id(0).unwrap();

        let updated_app = test_app.remove_with_id(target_id).unwrap();

        assert_eq!(updated_app.relationship_list.len(), 0);
    }

    #[test]
    fn remove_relationship_by_id_not_found(){
        let test_app = ClearHeadApp::default();

        let index_error = test_app.remove_with_id(Uuid::nil()).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_id_search_error());
    }

    #[test]
    fn select_by_index(){
        let test_app = create_app_with_single_relationship("related");

        let selected = RelationshipListManagement::select_by_index(&test_app, 0).unwrap();

        assert_eq!(selected, test_app.relationship_list[0]);
    }

    #[test]
    fn failed_select_by_index(){
        let empty_app = ClearHeadApp::default();

        let index_error = RelationshipListManagement::select_by_index(&empty_app, 0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn select_by_id(){
        let test_app = create_app_with_single_relationship("related");
        let target_id = test_app.get_id(0).unwrap();

        let selected = RelationshipListManagement::select_by_id(&test_app, target_id).unwrap();

        assert_eq!(selected, test_app.relationship_list[0]);
    }

    #[test]
    fn failed_select_by_id(){
        let empty_app = ClearHeadApp::default();

        let index_error = RelationshipListManagement::select_by_id(&empty_app, Uuid::nil()).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_id_search_error());
    }

    #[test]
    fn update_participant_1(){
        let test_app = create_app_with_single_relationship("related");
        let test_id = Uuid::new_v4();

        let updated_app = test_app.update_participant_1(0, test_id).unwrap();

        assert_eq!(updated_app.get_participant_1(0).unwrap(), test_id);
    }

    #[test]
    fn failed_update_participant_1(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.update_participant_1(0, Uuid::nil()).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn update_participant_2(){
        let test_app = create_app_with_single_relationship("related");
        let test_id = Uuid::new_v4();

        let updated_app = test_app.update_participant_2(0, test_id).unwrap();

        assert_eq!(updated_app.get_participant_2(0).unwrap(), test_id);
    }

    #[test]
    fn update_variant(){
        let test_app = create_app_with_single_relationship("related");

        let updated_app = test_app.change_variant(0, "sequential").unwrap();

        assert_eq!(updated_app.get_variant(0).unwrap(), RelationshipVariant::create_sequential());
    }

    #[test]
    fn change_to_parental(){
        let test_app = create_app_with_single_relationship("related");

        let updated_app = test_app.change_variant(0, "parental").unwrap();

        assert_eq!(updated_app.get_variant(0).unwrap(), RelationshipVariant::create_parental());
    }

    #[test]
    fn change_to_related(){
        let test_app = create_app_with_single_relationship("sequential");

        let updated_app = test_app.change_variant(0, "related").unwrap();

        assert_eq!(updated_app.get_variant(0).unwrap(), RelationshipVariant::create_related());
    }

    #[test]
    fn failed_change_variant_bad_index(){
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.change_variant(0, "related").unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn failed_change_variant_bad_variant(){
        let test_app = create_app_with_single_relationship("related");

        let index_error = test_app.change_variant(0, "bad_variant").unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_variant_error());
    }
}
