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

    fn get_id(&self, index: usize) -> Result<Uuid, String>;
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

    fn get_id(&self, index: usize) -> Result<Uuid, String> {
        let cloned_relationship = self.select_by_index(index)?;

        Ok(cloned_relationship.get_id())
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
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Configure, Token};
    use relationships::tests::create_nil_relationship;

    pub fn create_vector_with_nill_relationship() -> Vector<Relationship> {
        let mut list = Vector::new();
        let relationship = create_nil_relationship();

        list.push_back(relationship);

        list
    }

    #[test]
    fn serialize_and_deserialize() {
        let mut single_list = Vector::new();
        single_list.push_back(create_nil_relationship());

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
