pub mod relationships;
pub mod storage;

pub use crate::relationships::Relationship;
pub use crate::relationships::RelationshipManagement;

use crate::relationships::RelationshipVariant;
use crate::relationships::RelationshipVariantManagement;
use std::error::Error;
use uuid::Uuid;

use im::Vector;

pub trait RelationshipListManagement {
    type L: RelationshipListManagement;
    type V: RelationshipVariantManagement;
    fn add_new(
        &self,
        target_variant: &str,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Result<Self::L, Box<dyn Error>>;
    fn add_related(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L;
    fn add_sequential(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L;
    fn add_parental(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L;

    fn get_index_from_id(&self, id: Uuid) -> Result<usize, String>;
    fn get_id_from_index(&self, index: usize) -> Result<Uuid, String>;
    fn get_variant(&self, id: Uuid) -> Result<Self::V, String>;
    fn get_participant_1(&self, id: Uuid) -> Result<Uuid, String>;
    fn get_participant_2(&self, id: Uuid) -> Result<Uuid, String>;

    fn remove_at_index(&self, index: usize) -> Self::L;
    fn remove_with_id(&self, id: Uuid) -> Result<Self::L, String>;

    fn change_variant(&self, index: usize, variant: &str) -> Result<Self::L, String>;
    fn update_participant_1(&self, id: Uuid, new_id: Uuid) -> Result<Self::L, String>;
    fn update_participant_2(&self, id: Uuid, new_id: Uuid) -> Result<Self::L, String>;
}

impl RelationshipListManagement for Vector<Relationship> {
    type L = Vector<Relationship>;
    type V = RelationshipVariant;

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

    fn get_index_from_id(&self, id: Uuid) -> Result<usize, String> {
        let cloned_list = self.clone();

        return cloned_list
            .iter()
            .position(|relationship| relationship.get_id() == id)
            .ok_or("cannot find this id within the relationship list".to_string());
    }

    fn get_id_from_index(&self, index: usize) -> Result<Uuid, String> {
        Ok(self
            .get(index)
            .ok_or("Unable to find relationship at given index")?
            .get_id())
    }

    fn get_participant_1(&self, id: Uuid) -> Result<Uuid, String> {
        let index = self.get_index_from_id(id)?;

        Ok(self[index].get_participant_1())
    }

    fn get_participant_2(&self, id: Uuid) -> Result<Uuid, String> {
        let index = self.get_index_from_id(id)?;

        Ok(self[index].get_participant_2())
    }
    fn get_variant(&self, id: Uuid) -> Result<RelationshipVariant, String> {
        let index = self.get_index_from_id(id)?;

        return Ok(self[index].get_variant());
    }

    fn remove_with_id(&self, id: Uuid) -> Result<Self::L, String> {
        let cloned_list = self.clone();
        let target_index = cloned_list.get_index_from_id(id)?;

        let updated_list = cloned_list.remove_at_index(target_index);

        return Ok(updated_list);
    }

    fn change_variant(&self, index: usize, variant: &str) -> Result<Self::L, String> {
        let updated_relationship = self[index].set_variant(variant)?;
        let mut cloned_list = self.clone();

        cloned_list.set(index, updated_relationship);

        return Ok(cloned_list);
    }

    fn update_participant_1(&self, id: Uuid, new_id: Uuid) -> Result<Self::L, String> {
        let index = self.get_index_from_id(id)?;
        let updated_relationship = self[index].set_participant_1(new_id);
        let mut cloned_list = self.clone();

        cloned_list.set(index, updated_relationship);

        return Ok(cloned_list);
    }

    fn update_participant_2(&self, id: Uuid, new_id: Uuid) -> Result<Self::L, String> {
        let index = self.get_index_from_id(id)?;
        let updated_relationship = self[index].set_participant_2(new_id);
        let mut cloned_list = self.clone();

        cloned_list.set(index, updated_relationship);

        return Ok(cloned_list);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::relationships::edge_direction::EdgeDirectionality;
    use relationships::tests::create_nil_relationship;
    use serde_test::{assert_tokens, Configure, Token};

    pub fn add_nil_relationship_to_vector(list: Vector<Relationship>) -> Vector<Relationship> {
        let nil_relationship = create_nil_relationship(
            RelationshipVariant::Related(EdgeDirectionality::Undirected),
            Uuid::nil(),
            Uuid::nil(),
        );
        let mut cloned_list = list.clone();

        cloned_list.push_back(nil_relationship);

        return cloned_list;
    }

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
            .get_index_from_id(relationship_list[0].get_id())
            .unwrap();

        assert!(relationship_id == 0)
    }

    #[test]
    fn fail_to_find_id() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let failed_id_query = relationship_list
            .get_index_from_id(Uuid::nil())
            .unwrap_err();

        assert!(failed_id_query == "cannot find this id within the relationship list".to_string())
    }

    #[test]
    fn successfully_get_id() {
        let test_list: Vector<Relationship> = Vector::new();
        let single_relationship_list = test_list.add_related(Uuid::nil(), Uuid::nil());

        let id = single_relationship_list.get_id_from_index(0).unwrap();

        assert!(id == single_relationship_list[0].get_id())
    }

    #[test]
    fn failed_get_id() {
        let test_list: Vector<Relationship> = Vector::new();

        let extraction_error = test_list.get_id_from_index(0).unwrap_err();

        assert!(extraction_error == "Unable to find relationship at given index")
    }

    #[test]
    fn successfully_get_variant() {
        let test_list: Vector<Relationship> = Vector::new();
        let single_relationship_list = test_list.add_related(Uuid::new_v4(), Uuid::new_v4());

        let variant = single_relationship_list
            .get_variant(single_relationship_list[0].get_id())
            .unwrap();

        assert!(variant == RelationshipVariant::Related(EdgeDirectionality::Undirected));
    }

    #[test]
    fn failed_get_variant() {
        let test_list: Vector<Relationship> = Vector::new();

        let id_error = test_list.get_variant(Uuid::nil()).unwrap_err();

        assert!(id_error == "cannot find this id within the relationship list");
    }

    #[test]
    fn successfully_get_participant_1() {
        let test_list: Vector<Relationship> = Vector::new();
        let single_relationship_list = test_list.add_related(Uuid::nil(), Uuid::nil());

        let participant_1 = single_relationship_list
            .get_participant_1(single_relationship_list[0].get_id())
            .unwrap();

        assert!(participant_1 == single_relationship_list[0].get_participant_1())
    }

    #[test]
    fn failed_get_participant_1() {
        let test_list: Vector<Relationship> = Vector::new();

        let id_error = test_list.get_participant_1(Uuid::nil()).unwrap_err();

        assert!(id_error == "cannot find this id within the relationship list")
    }

    #[test]
    fn successfully_get_participant_2() {
        let test_list: Vector<Relationship> = Vector::new();
        let single_relationship_list = test_list.add_parental(Uuid::nil(), Uuid::nil());

        let participant_2 = single_relationship_list
            .get_participant_2(single_relationship_list[0].get_id())
            .unwrap();

        assert!(participant_2 == single_relationship_list[0].get_participant_2())
    }

    #[test]
    fn failed_get_participant_2() {
        let test_list: Vector<Relationship> = Vector::new();

        let empty_list_error = test_list.get_participant_2(Uuid::nil()).unwrap_err();

        assert!(empty_list_error == "cannot find this id within the relationship list")
    }

    #[test]
    fn successfully_remove_from_id() {
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

    #[test]
    fn change_related_to_parental() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let test_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        let updated_list = test_list.change_variant(0, "parental").unwrap();

        assert!(
            updated_list[0].get_variant()
                == RelationshipVariant::Parental(EdgeDirectionality::Directed)
        );
    }

    #[test]
    fn change_parental_to_related() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let test_list = relationship_list.add_parental(Uuid::nil(), Uuid::nil());

        let updated_list = test_list.change_variant(0, "related").unwrap();

        assert!(
            updated_list[0].get_variant()
                == RelationshipVariant::Related(EdgeDirectionality::Undirected)
        )
    }

    #[test]
    fn failed_change_variant() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let test_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        let failed_output = test_list.change_variant(0, "bad variant").unwrap_err();

        assert!(failed_output == "invalid relationship variant")
    }

    #[test]
    fn update_participant_1_id() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let test_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        let updated_list = test_list
            .update_participant_1(test_list[0].get_id(), Uuid::new_v4())
            .unwrap();

        assert!(updated_list[0].get_participant_1() != Uuid::nil())
    }

    #[test]
    fn failed_id_update_participant_1() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let test_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        let bad_list_search = test_list
            .update_participant_1(Uuid::new_v4(), Uuid::new_v4())
            .unwrap_err();

        assert!(bad_list_search == "cannot find this id within the relationship list")
    }

    #[test]
    fn update_participant_2_id() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let test_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        let updated_list = test_list
            .update_participant_2(test_list[0].get_id(), Uuid::new_v4())
            .unwrap();

        assert!(updated_list[0].get_participant_2() != Uuid::nil())
    }

    #[test]
    fn failed_id_update_participant_2() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let test_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        let bad_list_search = test_list
            .update_participant_2(Uuid::new_v4(), Uuid::new_v4())
            .unwrap_err();

        assert!(bad_list_search == "cannot find this id within the relationship list")
    }

    #[test]
    fn serialize_and_deserialize() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let single_list = add_nil_relationship_to_vector(relationship_list);
        assert_tokens(
            &single_list.readable(),
            &[
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Relationship",
                    len: 4,
                },
                Token::Str("id"),
                Token::Str("00000000-0000-0000-0000-000000000000"),
                Token::Str("variant"),
                Token::NewtypeVariant {
                    name: "RelationshipVariant",
                    variant: "Related",
                },
                Token::UnitVariant {
                    name: "EdgeDirectionality",
                    variant: "Undirected",
                },
                Token::Str("participant_1"),
                Token::Str("00000000-0000-0000-0000-000000000000"),
                Token::Str("participant_2"),
                Token::Str("00000000-0000-0000-0000-000000000000"),
                Token::StructEnd,
                Token::SeqEnd,
            ],
        )
    }
}
