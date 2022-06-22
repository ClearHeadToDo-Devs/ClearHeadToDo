pub mod relationships;
pub use crate::relationships::Relationship;
pub use crate::relationships::RelationshipManagement;

use std::error::Error;
use uuid::Uuid;

use im::Vector;

trait RelationshipListManagement {
    type L: RelationshipListManagement;
    fn add_new(
        &self,
        target_variant: &str,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Result<Self::L, Box<dyn Error>>;
    fn add_related(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L;
    fn add_sequential(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L;
    fn add_parental(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L;

    fn return_index_from_id(&self, id: Uuid) -> Result<usize, String>;

    fn remove_at_index(&self, index: usize) -> Self::L;
    fn remove_with_id(&self, id: Uuid) -> Result<Self::L, String>;
}

impl RelationshipListManagement for Vector<Relationship> {
    type L = Vector<Relationship>;

    fn add_new(
        &self,
        target_variant: &str,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Result<Vector<Relationship>, Box<dyn Error>> {
        let mut cloned_list = self.clone();
        let new_relationship =
            Relationship::create_new(target_variant, participant_1, participant_2)?;

        cloned_list.push_back(new_relationship);
        return Ok(cloned_list);
    }

    fn add_related(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let new_relationship = Relationship::create_new_related(participant_1, participant_2);
        let mut cloned_list = self.clone();

        cloned_list.push_back(new_relationship);

        return cloned_list;
    }

    fn add_sequential(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let new_relationship = Relationship::create_new_sequential(participant_1, participant_2);
        let mut cloned_list = self.clone();

        cloned_list.push_back(new_relationship);

        return cloned_list;
    }

    fn add_parental(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let new_relationship = Relationship::create_new_parental(participant_1, participant_2);
        let mut cloned_list = self.clone();

        cloned_list.push_back(new_relationship);

        return cloned_list;
    }

    fn remove_at_index(&self, index: usize) -> Self::L {
        let mut updated_list = self.clone();
        updated_list.remove(index);
        return updated_list;
    }

    fn return_index_from_id(&self, id: Uuid) -> Result<usize, String> {
        let cloned_list = self.clone();

        return cloned_list
            .iter()
            .position(|relationship| relationship.get_id() == id)
            .ok_or("cannot find this id within the relationship list".to_string());
    }

    fn remove_with_id(&self, id: Uuid) -> Result<Self::L, String> {
        let cloned_list = self.clone();
        let target_index = cloned_list.return_index_from_id(id)?;

        let updated_list = cloned_list.remove_at_index(target_index);

        return Ok(updated_list);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_new_from_string() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let variant_string = "related".to_string();

        let updated_list = relationship_list
            .add_new(&variant_string, Uuid::nil(), Uuid::nil())
            .unwrap();

        assert!(updated_list.len() == 1)
    }

    #[test]
    fn add_related_to_list() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let modified_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        assert! {modified_list[0].get_variant().to_string() == "Related: Undirected"}
    }

    #[test]
    fn add_sequential_to_list() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let modified_list = relationship_list.add_sequential(Uuid::nil(), Uuid::nil());

        assert! {modified_list[0].get_variant().to_string() == "Sequential: Directed"}
    }

    #[test]
    fn add_parental_to_list() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let modified_list = relationship_list.add_parental(Uuid::nil(), Uuid::nil());

        assert!(modified_list[0].get_variant().to_string() == "Parental: Directed")
    }

    #[test]
    fn remove_relationship() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let modified_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        let pruned_list = modified_list.remove_at_index(0);

        assert!(pruned_list.len() == 0);
    }

    #[test]
    #[should_panic]
    fn empty_vector_removal_error() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let failed_poped_list = relationship_list.remove_at_index(0);

        assert!(failed_poped_list.len() == 0)
    }

    #[test]
    fn return_index_from_id() {
        let relationship_list: Vector<Relationship> =
            Vector::new().add_related(Uuid::nil(), Uuid::nil());

        let relationship_id = relationship_list
            .return_index_from_id(relationship_list[0].get_id())
            .unwrap();

        assert!(relationship_id == 0)
    }

    #[test]
    fn fail_to_find_id() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let failed_id_query = relationship_list
            .return_index_from_id(Uuid::nil())
            .unwrap_err();

        assert!(failed_id_query == "cannot find this id within the relationship list".to_string())
    }

    #[test]
    fn remove_from_id() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let single_relationship_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        let empty_list = single_relationship_list
            .remove_with_id(single_relationship_list[0].get_id())
            .unwrap();

        assert!(empty_list.len() == 0)
    }

    #[test]
    fn failed_remove_from_id() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let failed_id_query = relationship_list.remove_with_id(Uuid::nil()).unwrap_err();

        assert!(failed_id_query == "cannot find this id within the relationship list".to_string())
    }
}
