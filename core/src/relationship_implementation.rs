use crate::ClearHeadApp;

use crate::relationship::item::RelationshipVariant;
use crate::relationship::Relationship;
use crate::relationship::RelationshipListManagement;
use im::Vector;

use std::error::Error;
use uuid::Uuid;

impl RelationshipListManagement for ClearHeadApp {
    type L = ClearHeadApp;
    fn append_new_relationship(
        &self,
        target_variant: &str,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Result<Self::L, Box<dyn Error>> {
        let mut updated_app = self.clone();

        let updated_relationship_list = self.relationship_list.append_new_relationship(
            target_variant,
            participant_1,
            participant_2,
        )?;

        updated_app.relationship_list = updated_relationship_list;
        Ok(updated_app)
    }
    fn append_related_relationship(
        &self,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> ClearHeadApp {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self
            .relationship_list
            .append_related_relationship(participant_1, participant_2);

        cloned_app.relationship_list = updated_relationship_list;
        return cloned_app;
    }
    fn append_sequential_relationship(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self
            .relationship_list
            .append_sequential_relationship(participant_1, participant_2);

        cloned_app.relationship_list = updated_relationship_list;
        return cloned_app;
    }
    fn append_parental_relationship(
        &self,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> ClearHeadApp {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self
            .relationship_list
            .append_parental_relationship(participant_1, participant_2);

        cloned_app.relationship_list = updated_relationship_list;
        return cloned_app;
    }

    fn select_relationship_by_id(&self, id: Uuid) -> Result<Relationship, String> {
        Ok(self.relationship_list.select_relationship_by_id(id)?)
    }
    fn select_relationship_by_index(&self, index: usize) -> Result<Relationship, Box<dyn Error>> {
        Ok(self.relationship_list.select_relationship_by_index(index)?)
    }

    fn get_relationship_id(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        Ok(self.relationship_list.get_relationship_id(index)?)
    }
    fn get_relationship_variant(
        &self,
        index: usize,
    ) -> Result<RelationshipVariant, Box<dyn Error>> {
        Ok(self.relationship_list.get_relationship_variant(index)?)
    }
    fn get_relationship_participant_1(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        Ok(self
            .relationship_list
            .get_relationship_participant_1(index)?)
    }
    fn get_relationship_participant_2(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        Ok(self
            .relationship_list
            .get_relationship_participant_2(index)?)
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

    fn change_relationship_variant(
        &self,
        index: usize,
        variant: &str,
    ) -> Result<Self::L, Box<dyn Error>> {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self
            .relationship_list
            .change_relationship_variant(index, variant)?;
        cloned_app.relationship_list = updated_relationship_list;
        Ok(cloned_app)
    }
    fn update_relationship_participant_1(
        &self,
        index: usize,
        new_id: Uuid,
    ) -> Result<Self::L, Box<dyn Error>> {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self
            .relationship_list
            .update_relationship_participant_1(index, new_id)?;
        cloned_app.relationship_list = updated_relationship_list;
        Ok(cloned_app)
    }
    fn update_relationship_participant_2(
        &self,
        index: usize,
        new_id: Uuid,
    ) -> Result<Self::L, Box<dyn Error>> {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self
            .relationship_list
            .update_relationship_participant_2(index, new_id)?;
        cloned_app.relationship_list = updated_relationship_list;
        Ok(cloned_app)
    }

    fn id_is_present_in_participant_1_list(&self, id: Uuid) -> bool {
        self.relationship_list
            .id_is_present_in_participant_1_list(id)
    }
    fn id_is_present_in_participant_2_list(&self, id: Uuid) -> bool {
        self.relationship_list
            .id_is_present_in_participant_2_list(id)
    }
    fn id_is_present_in_either_participant_list(&self, id: Uuid) -> bool {
        self.relationship_list
            .id_is_present_in_either_participant_list(id)
    }

    fn get_children_for_id(&self, id: Uuid) -> Result<Vector<Uuid>, Box<dyn Error>> {
        todo!()
    }
    fn get_participant_1_list_for_id(
        &self,
        id: Uuid,
    ) -> Result<Vector<Relationship>, Box<dyn Error>> {
        Ok(self.relationship_list.get_participant_1_list_for_id(id)?)
    }
    fn get_participant_2_list_for_id(
        &self,
        id: Uuid,
    ) -> Result<Vector<Relationship>, Box<dyn Error>> {
        Ok(self.relationship_list.get_participant_2_list_for_id(id)?)
    }
    fn get_either_participant_list_for_id(
        &self,
        id: Uuid,
    ) -> Result<Vector<Relationship>, Box<dyn Error>> {
        Ok(self
            .relationship_list
            .get_either_participant_list_for_id(id)?)
    }

    fn filter_by_participants(
        &self,
        participant_list: String,
        id: Uuid,
    ) -> Result<Self::L, Box<dyn Error>> {
        let mut cloned_app = self.clone();

        let updated_relationship_list = self
            .relationship_list
            .filter_by_participants(participant_list, id)?;
        cloned_app.relationship_list = updated_relationship_list;

        Ok(cloned_app)
    }
    fn filter_by_variant(&self, variant: &str) -> Result<Vector<Relationship>, Box<dyn Error>> {
        todo!()
    }

    fn get_relationship_list_as_table(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{functionality::tests::failed_relationship_index_error, ClearHeadApp};

    use super::*;

    pub fn create_app_with_single_relationship(variant_str: &str) -> ClearHeadApp {
        ClearHeadApp::default()
            .append_new_relationship(variant_str, Uuid::nil(), Uuid::nil())
            .unwrap()
    }

    pub fn failed_relationship_id_search_error() -> String {
        "cannot find this id within the relationship list".to_string()
    }

    pub fn failed_relationship_variant_error() -> String {
        "invalid relationship variant".to_string()
    }
    #[test]
    fn create_relationship() {
        let test_app = ClearHeadApp::default();

        let updated_app = test_app
            .append_new_relationship("related", Uuid::nil(), Uuid::nil())
            .unwrap();

        assert_eq!(updated_app.relationship_list.len(), 1);
    }

    #[test]
    fn failed_create_relationship() {
        let test_app = ClearHeadApp::default();

        let bad_variant_error = test_app
            .append_new_relationship("invalid", Uuid::nil(), Uuid::nil())
            .unwrap_err();

        assert_eq!(
            bad_variant_error.to_string(),
            failed_relationship_variant_error()
        );
    }

    #[test]
    fn create_related() {
        let test_app = ClearHeadApp::default();

        let updated_app = test_app
            .append_new_relationship("related", Uuid::nil(), Uuid::nil())
            .unwrap();

        assert_eq!(updated_app.relationship_list.len(), 1);
    }

    #[test]
    fn create_sequential() {
        let test_app = ClearHeadApp::default();

        let updated_app = test_app
            .append_new_relationship("sequential", Uuid::nil(), Uuid::nil())
            .unwrap();

        assert_eq!(updated_app.relationship_list.len(), 1);
    }

    #[test]
    fn create_parental() {
        let test_app = ClearHeadApp::default();

        let updated_app = test_app
            .append_new_relationship("parental", Uuid::nil(), Uuid::nil())
            .unwrap();

        assert_eq!(
            updated_app.get_relationship_variant(0).unwrap(),
            RelationshipVariant::create_parental()
        );
    }

    #[test]
    fn create_related_direct() {
        let test_app = ClearHeadApp::default();

        let updated_app = test_app.append_related_relationship(Uuid::nil(), Uuid::nil());

        assert_eq!(
            updated_app.get_relationship_variant(0).unwrap(),
            RelationshipVariant::create_related()
        );
    }

    #[test]
    fn create_sequential_direct() {
        let test_app = ClearHeadApp::default();

        let updated_app = test_app.append_sequential_relationship(Uuid::nil(), Uuid::nil());

        assert_eq!(
            updated_app.get_relationship_variant(0).unwrap(),
            RelationshipVariant::create_sequential()
        );
    }

    #[test]
    fn create_parental_direct() {
        let test_app = ClearHeadApp::default();

        let updated_app = test_app.append_parental_relationship(Uuid::nil(), Uuid::nil());

        assert_eq!(
            updated_app.get_relationship_variant(0).unwrap(),
            RelationshipVariant::create_parental()
        );
    }

    #[test]
    fn get_variant() {
        let test_app = create_app_with_single_relationship("related");

        let variant = test_app.get_relationship_variant(0).unwrap();

        assert_eq!(variant, RelationshipVariant::create_related());
    }

    #[test]
    fn failed_get_variant() {
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.get_relationship_variant(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn get_id() {
        let test_app = create_app_with_single_relationship("related");

        let id = test_app.get_relationship_id(0).unwrap();

        assert_eq!(
            id,
            test_app.relationship_list.get_relationship_id(0).unwrap()
        );
    }

    #[test]
    fn failed_get_id() {
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.get_relationship_id(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn get_participant_1() {
        let test_app = create_app_with_single_relationship("related");

        let participant_1 = test_app.get_relationship_participant_1(0).unwrap();

        assert_eq!(participant_1, Uuid::nil());
    }

    #[test]
    fn failed_get_participant_1() {
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.get_relationship_participant_1(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn get_participant_2() {
        let test_app = create_app_with_single_relationship("related");

        let participant_2 = test_app.get_relationship_participant_2(0).unwrap();

        assert_eq!(participant_2, Uuid::nil());
    }

    #[test]
    fn failed_get_participant_2() {
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.get_relationship_participant_2(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn remove_relationship() {
        let test_app = create_app_with_single_relationship("related");

        let updated_app = test_app.remove_at_index(0).unwrap();

        assert_eq!(updated_app.relationship_list.len(), 0);
    }

    #[test]
    fn failed_remove_relationship() {
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app.remove_at_index(0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn remove_relationship_by_id() {
        let test_app = create_app_with_single_relationship("related");
        let target_id = test_app.get_relationship_id(0).unwrap();

        let updated_app = test_app.remove_with_id(target_id).unwrap();

        assert_eq!(updated_app.relationship_list.len(), 0);
    }

    #[test]
    fn remove_relationship_by_id_not_found() {
        let test_app = ClearHeadApp::default();

        let index_error = test_app.remove_with_id(Uuid::nil()).unwrap_err();

        assert_eq!(
            index_error.to_string(),
            failed_relationship_id_search_error()
        );
    }

    #[test]
    fn select_by_index() {
        let test_app = create_app_with_single_relationship("related");

        let selected =
            RelationshipListManagement::select_relationship_by_index(&test_app, 0).unwrap();

        assert_eq!(selected, test_app.relationship_list[0]);
    }

    #[test]
    fn failed_select_by_index() {
        let empty_app = ClearHeadApp::default();

        let index_error =
            RelationshipListManagement::select_relationship_by_index(&empty_app, 0).unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn select_by_id() {
        let test_app = create_app_with_single_relationship("related");
        let target_id = test_app.get_relationship_id(0).unwrap();

        let selected =
            RelationshipListManagement::select_relationship_by_id(&test_app, target_id).unwrap();

        assert_eq!(selected, test_app.relationship_list[0]);
    }

    #[test]
    fn failed_select_by_id() {
        let empty_app = ClearHeadApp::default();

        let index_error =
            RelationshipListManagement::select_relationship_by_id(&empty_app, Uuid::nil())
                .unwrap_err();

        assert_eq!(
            index_error.to_string(),
            failed_relationship_id_search_error()
        );
    }

    #[test]
    fn update_participant_1() {
        let test_app = create_app_with_single_relationship("related");
        let test_id = Uuid::new_v4();

        let updated_app = test_app
            .update_relationship_participant_1(0, test_id)
            .unwrap();

        assert_eq!(
            updated_app.get_relationship_participant_1(0).unwrap(),
            test_id
        );
    }

    #[test]
    fn failed_update_participant_1() {
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app
            .update_relationship_participant_1(0, Uuid::nil())
            .unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn update_participant_2() {
        let test_app = create_app_with_single_relationship("related");
        let test_id = Uuid::new_v4();

        let updated_app = test_app
            .update_relationship_participant_2(0, test_id)
            .unwrap();

        assert_eq!(
            updated_app.get_relationship_participant_2(0).unwrap(),
            test_id
        );
    }

    #[test]
    fn update_variant() {
        let test_app = create_app_with_single_relationship("related");

        let updated_app = test_app
            .change_relationship_variant(0, "sequential")
            .unwrap();

        assert_eq!(
            updated_app.get_relationship_variant(0).unwrap(),
            RelationshipVariant::create_sequential()
        );
    }

    #[test]
    fn change_to_parental() {
        let test_app = create_app_with_single_relationship("related");

        let updated_app = test_app.change_relationship_variant(0, "parental").unwrap();

        assert_eq!(
            updated_app.get_relationship_variant(0).unwrap(),
            RelationshipVariant::create_parental()
        );
    }

    #[test]
    fn change_to_related() {
        let test_app = create_app_with_single_relationship("sequential");

        let updated_app = test_app.change_relationship_variant(0, "related").unwrap();

        assert_eq!(
            updated_app.get_relationship_variant(0).unwrap(),
            RelationshipVariant::create_related()
        );
    }

    #[test]
    fn failed_change_variant_bad_index() {
        let empty_app = ClearHeadApp::default();

        let index_error = empty_app
            .change_relationship_variant(0, "related")
            .unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_index_error());
    }

    #[test]
    fn failed_change_variant_bad_variant() {
        let test_app = create_app_with_single_relationship("related");

        let index_error = test_app
            .change_relationship_variant(0, "bad_variant")
            .unwrap_err();

        assert_eq!(index_error.to_string(), failed_relationship_variant_error());
    }

    #[test]
    fn check_id_in_participant_1() {
        let test_app = create_app_with_single_relationship("related");
        let test_id = test_app.get_relationship_participant_1(0).unwrap();

        let result = test_app.id_is_present_in_participant_1_list(test_id);

        assert!(result == true);
    }

    #[test]
    fn check_id_not_in_participant_1() {
        let test_app = create_app_with_single_relationship("related");
        let test_id = Uuid::new_v4();

        let result = test_app.id_is_present_in_participant_1_list(test_id);

        assert!(result == false);
    }

    #[test]
    fn check_id_in_participant_2() {
        let test_app = create_app_with_single_relationship("related");
        let test_id = test_app.get_relationship_participant_2(0).unwrap();

        let result = test_app.id_is_present_in_participant_2_list(test_id);

        assert!(result == true);
    }

    #[test]
    fn check_id_not_in_participant_2() {
        let test_app = create_app_with_single_relationship("related");
        let test_id = Uuid::new_v4();

        let result = test_app.id_is_present_in_participant_2_list(test_id);

        assert!(result == false);
    }

    #[test]
    fn check_id_in_either_participant_lists() {
        let test_app = create_app_with_single_relationship("related");
        let test_id = test_app.get_relationship_participant_1(0).unwrap();

        let result = test_app.id_is_present_in_either_participant_list(test_id);

        assert!(result == true);
    }

    #[test]
    fn check_id_not_in_either_participant_lists() {
        let test_app = create_app_with_single_relationship("related");
        let test_id = Uuid::new_v4();

        let result = test_app.id_is_present_in_either_participant_list(test_id);

        assert!(result == false);
    }

    #[test]
    fn get_participant_1_list_for_id() {
        let test_app =
            ClearHeadApp::default().append_related_relationship(Uuid::nil(), Uuid::new_v4());

        let result = test_app.get_participant_1_list_for_id(Uuid::nil()).unwrap();

        assert_eq!(
            result.select_relationship_by_index(0).unwrap(),
            test_app.select_relationship_by_index(0).unwrap()
        );
    }

    #[test]
    fn failed_get_participant_1_list_for_id() {
        let test_app = ClearHeadApp::default();

        let result = test_app
            .get_participant_1_list_for_id(Uuid::nil())
            .unwrap_err();

        assert_eq!(
            result.to_string(),
            "Unable to find Relationship with given Id in participant 1 list"
        );
    }

    #[test]
    fn get_participant_2_list_for_id() {
        let test_app =
            ClearHeadApp::default().append_related_relationship(Uuid::new_v4(), Uuid::nil());

        let result = test_app.get_participant_2_list_for_id(Uuid::nil()).unwrap();

        assert_eq!(
            result.select_relationship_by_index(0).unwrap(),
            test_app.select_relationship_by_index(0).unwrap()
        );
    }

    #[test]
    fn failed_get_participant_2_list_for_id() {
        let test_app = ClearHeadApp::default();

        let result = test_app
            .get_participant_2_list_for_id(Uuid::nil())
            .unwrap_err();

        assert_eq!(
            result.to_string(),
            "Unable to find Relationship with given Id in participant 2 list"
        );
    }

    #[test]
    fn get_either_participant_list_for_id() {
        let test_app = ClearHeadApp::default()
            .append_related_relationship(Uuid::new_v4(), Uuid::nil())
            .append_related_relationship(Uuid::nil(), Uuid::new_v4());

        let result = test_app
            .get_either_participant_list_for_id(Uuid::nil())
            .unwrap();

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn failed_get_either_participant_list_for_id() {
        let test_app = ClearHeadApp::default();

        let result = test_app
            .get_either_participant_list_for_id(Uuid::nil())
            .unwrap_err();

        assert_eq!(
            result.to_string(),
            "Unable to find Relationship with given Id in either participant list"
        );
    }
}
