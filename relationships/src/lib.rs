pub mod relationships;

pub mod storage;

pub use crate::relationships::Relationship;

use crate::relationships::RelationshipVariant;

use std::error::Error;
use uuid::Uuid;

use im::Vector;

pub trait RelationshipListManagement {
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

    fn select_by_id(&self, id: Uuid) -> Result<Relationship, String>;
    fn select_by_index(&self, index: usize) -> Result<Relationship, String>;
    fn get_variant(&self, index: usize) -> Result<RelationshipVariant, String>;
    fn get_participant_1(&self, index: usize) -> Result<Uuid, String>;
    fn get_participant_2(&self, index: usize) -> Result<Uuid, String>;

    fn remove_at_index(&self, index: usize) -> Self::L;
    fn remove_with_id(&self, id: Uuid) -> Result<Self::L, String>;

    fn change_variant(&self, index: usize, variant: &str) -> Result<Self::L, String>;
    fn update_participant_1(&self, index: usize, new_id: Uuid) -> Result<Self::L, String>;
    fn update_participant_2(&self, index: usize, new_id: Uuid) -> Result<Self::L, String>;
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
        let new_relationship = Relationship::create_new(
            target_variant, participant_1, participant_2)?;

        cloned_list.push_back(new_relationship);
        return Ok(cloned_list);
    }

    fn add_related(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let mut cloned_list = self.clone();
        let new_relationship = Relationship::create_new_related(
            participant_1, participant_2);

        cloned_list.push_back(new_relationship);

        return cloned_list;
    }

    fn add_sequential(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let mut cloned_list = self.clone();
        let new_relationship = Relationship::create_new_sequential(participant_1, participant_2);

        cloned_list.push_back(new_relationship);

        return cloned_list;
    }

    fn add_parental(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let mut cloned_list = self.clone();
        let new_relationship = Relationship::create_new_parental(participant_1, participant_2);

        cloned_list.push_back(new_relationship);

        return cloned_list;
    }

    fn remove_at_index(&self, index: usize) -> Self::L {
        let mut updated_list = self.clone();
        updated_list.remove(index);
        return updated_list;
    }

    fn select_by_id(&self, id: Uuid) -> Result<Relationship, String> {
        let query_result = self.iter().find(|relationship| relationship.get_id() == id)
            .ok_or("cannot find this id within the relationship list".to_string());

        let relationship_clone = query_result?.clone();

        return Ok(relationship_clone);
    }

    fn select_by_index(&self, index: usize) -> Result<Relationship, String> {
        let relationship_ref = self.get(index)
            .ok_or("Unable to find relationship at given index")?;

        let cloned_relationship = relationship_ref.clone();

        Ok(cloned_relationship)
    }

    fn get_participant_1(&self, index: usize) -> Result<Uuid, String> {
        let relationship_clone = self.select_by_index(index)?;

        Ok(relationship_clone.get_participant_1())
    }

    fn get_participant_2(&self, index: usize) -> Result<Uuid, String> {
        let cloned_relationship = self.select_by_index(index)?;

        Ok(cloned_relationship.get_participant_2())
    }
    fn get_variant(&self, index: usize) -> Result<RelationshipVariant, String> {
        let cloned_relationship = self.select_by_index(index)?;

        Ok(cloned_relationship.get_variant())
    }

    fn remove_with_id(&self, id: Uuid) -> Result<Self::L, String> {
        let cloned_list = self.clone();
        let target_index = cloned_list.index_of(&cloned_list.select_by_id(id)?)
            .ok_or("Unable to find relationship with given id")?;

        let updated_list = cloned_list.remove_at_index(target_index);

        return Ok(updated_list);
    }

    fn change_variant(&self, index: usize, variant: &str) -> Result<Self::L, String> {
        let updated_relationship = self[index].set_variant(variant)?;
        let mut cloned_list = self.clone();

        cloned_list.set(index, updated_relationship);

        return Ok(cloned_list);
    }

    fn update_participant_1(&self, index: usize, new_id: Uuid) -> Result<Self::L, String> {
        let mut cloned_list = self.clone();
        let cloned_relationship = self.select_by_index(index)?;

        let updated_relationship = cloned_relationship.set_participant_1(new_id);

        cloned_list.set(index, updated_relationship);

        return Ok(cloned_list);
    }

    fn update_participant_2(&self, index: usize, new_id: Uuid) -> Result<Self::L, String> {
        let mut cloned_list = self.clone();
        let cloned_relationship = self.select_by_index(index)?;

        let updated_relationship = cloned_relationship.set_participant_2(new_id);
        cloned_list.set(index, updated_relationship);

        Ok(cloned_list)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::relationships::edge_direction::EdgeDirectionality;
    use relationships::tests::create_nil_relationship;
    use serde_test::{assert_tokens, Configure, Token};

    pub fn add_nil_relationship_to_vector(list: Vector<Relationship>) -> Vector<Relationship> {
        let mut cloned_list = list.clone();
        let nil_relationship = create_nil_relationship();

        cloned_list.push_back(nil_relationship);

        return cloned_list;
    }

    pub fn create_relationship_list_with_single_related_relationship() -> Vector<Relationship> {
        let mut list = Vector::new();
        let relationship = Relationship::create_new_related(
            Uuid::nil(),
            Uuid::nil(),
        );

        list.push_back(relationship);

        return list;
    }

    pub fn create_relationship_list_with_single_relationship(variant: &str) -> Vector<Relationship> {
        let mut list: Vector<Relationship> = Vector::new();

        let relationship = Relationship::create_new(variant,Uuid::nil(),Uuid::nil()).unwrap();

        list.push_back(relationship);

        return list;
    }

    #[test]
    fn create_new_from_string() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let variant_string = "related".to_string();

        let updated_list = relationship_list
            .add_new(&variant_string, Uuid::nil(), Uuid::nil())
            .unwrap();

        assert_eq!(updated_list[0].get_variant() , RelationshipVariant::create_related());
    }

    #[test]
    fn add_related_to_list() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let modified_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        assert_eq!(modified_list[0].get_variant() , RelationshipVariant::create_related());
    }

    #[test]
    fn add_sequential_to_list() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let modified_list = relationship_list.add_sequential(Uuid::nil(), Uuid::nil());

        assert_eq!(modified_list[0].get_variant() ,
            RelationshipVariant::create_sequential());
    }

    #[test]
    fn add_parental_to_list() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let modified_list = relationship_list.add_parental(Uuid::nil(), Uuid::nil());

        assert_eq!(modified_list[0].get_variant() , RelationshipVariant::create_parental());
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
    fn return_relationship_from_id() {
        let relationship_list: Vector<Relationship> =
            create_relationship_list_with_single_related_relationship();

        let relationship_id = relationship_list
            .select_by_id(relationship_list[0].get_id())
            .unwrap();

        assert!(relationship_id == relationship_list[0]);
    }

    #[test]
    fn fail_to_find_id() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let failed_id_query = relationship_list
            .select_by_id(Uuid::nil())
            .unwrap_err();

        assert!(failed_id_query == "cannot find this id within the relationship list".to_string())
    }

    #[test]
    fn successfully_get_id() {
        let relationship_list = create_relationship_list_with_single_related_relationship();

        let relationship = relationship_list.select_by_index(0).unwrap();

        assert_eq!(relationship, relationship_list[0]);
    }

    #[test]
    fn failed_get_id() {
        let test_list: Vector<Relationship> = Vector::new();

        let extraction_error = test_list.select_by_index(0).unwrap_err();

       assert_eq!(extraction_error , "Unable to find relationship at given index".to_string());
    }

    #[test]
    fn successfully_get_variant() {
        let relationship_list = create_relationship_list_with_single_related_relationship();

        let variant = relationship_list
            .get_variant(0)
            .unwrap();

        assert!(variant == RelationshipVariant::create_related())
    }

    #[test]
    fn failed_get_variant() {
        let test_list: Vector<Relationship> = Vector::new();

        let index_error = test_list.get_variant(0).unwrap_err();

        assert_eq!(index_error , "Unable to find relationship at given index".to_string());
    }

    #[test]
    fn successfully_get_participant_1() {
        let test_list: Vector<Relationship> = create_relationship_list_with_single_related_relationship();

        let participant_1 = test_list
            .get_participant_1(0)
            .unwrap();

        assert_eq!(participant_1, Uuid::nil());
    }

    #[test]
    fn failed_get_participant_1() {
        let test_list: Vector<Relationship> = Vector::new();

        let id_error = test_list.get_participant_1(0).unwrap_err();

        assert_eq!(id_error , "Unable to find relationship at given index".to_string());
    }

    #[test]
    fn successfully_get_participant_2() {
        let test_list: Vector<Relationship> = Vector::new();
        let single_relationship_list = test_list.add_parental(Uuid::nil(), Uuid::nil());

        let participant_2 = single_relationship_list
            .get_participant_2(0)
            .unwrap();

        assert!(participant_2 == single_relationship_list[0].get_participant_2())
    }

    #[test]
    fn failed_get_participant_2() {
        let test_list: Vector<Relationship> = Vector::new();

        let empty_list_error = test_list.get_participant_2(0).unwrap_err();

        assert_eq!(empty_list_error , "Unable to find relationship at given index".to_string());
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
            .update_participant_1(0, Uuid::new_v4())
            .unwrap();

        assert!(updated_list[0].get_participant_1() != Uuid::nil())
    }

    #[test]
    fn failed_id_update_participant_1() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let bad_list_search = relationship_list
            .update_participant_1(0, Uuid::new_v4())
            .unwrap_err();

        assert!(bad_list_search == "Unable to find relationship at given index")
    }

    #[test]
    fn update_participant_2_id() {
        let relationship_list: Vector<Relationship> = Vector::new();
        let test_list = relationship_list.add_related(Uuid::nil(), Uuid::nil());

        let updated_list = test_list
            .update_participant_2(0, Uuid::new_v4())
            .unwrap();

        assert!(updated_list[0].get_participant_2() != Uuid::nil())
    }

    #[test]
    fn failed_id_update_participant_2() {
        let relationship_list: Vector<Relationship> = Vector::new();

        let bad_list_search = relationship_list
            .update_participant_2(0, Uuid::new_v4())
            .unwrap_err();

        assert!(bad_list_search == "Unable to find relationship at given index")
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
