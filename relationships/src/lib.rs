pub mod item;

pub mod storage;

pub use crate::item::Relationship;

use crate::item::RelationshipVariant;

use std::error::Error;
use tabled::Table;
use uuid::Uuid;

use im::Vector;

pub trait RelationshipListManagement {
    type L: RelationshipListManagement;
    fn append_new_relationship(
        &self,
        target_variant: &str,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Result<Self::L, Box<dyn Error>>;
    fn append_related_relationship(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L;
    fn append_sequential_relationship(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L;
    fn append_parental_relationship(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L;

    fn select_relationship_by_id(&self, id: Uuid) -> Result<Relationship, String>;
    fn select_relationship_by_index(&self, index: usize) -> Result<Relationship, Box<dyn Error>>;

    fn get_relationship_id(&self, index: usize) -> Result<Uuid, Box<dyn Error>>;
    fn get_relationship_variant(&self, index: usize) -> Result<RelationshipVariant, Box<dyn Error>>;
    fn get_relationship_participant_1(&self, index: usize) -> Result<Uuid, Box<dyn Error>>;
    fn get_relationship_participant_2(&self, index: usize) -> Result<Uuid, Box<dyn Error>>;

    fn remove_at_index(&self, index: usize) -> Result<Self::L, Box<dyn Error>>;
    fn remove_with_id(&self, id: Uuid) -> Result<Self::L, Box<dyn Error>>;

    fn change_relationship_variant(&self, index: usize, variant: &str) -> Result<Self::L, Box<dyn Error>>;
    fn update_relationship_participant_1(&self, index: usize, new_id: Uuid) -> Result<Self::L, Box<dyn Error>>;
    fn update_relationship_participant_2(&self, index: usize, new_id: Uuid) -> Result<Self::L, Box<dyn Error>>;

    fn id_is_present_in_participant_1_list(&self, id: Uuid) -> bool;
    fn id_is_present_in_participant_2_list(&self, id: Uuid) -> bool;
    fn id_is_present_in_either_participant_list(&self, id: Uuid) -> bool;
    fn get_participant_1_list_for_id(&self, id: Uuid) -> Result<Vector<Relationship>, Box<dyn Error>>;
    fn get_participant_2_list_for_id(&self, id: Uuid) -> Result<Vector<Relationship>, Box<dyn Error>>;
    fn get_either_participant_list_for_id(&self, id: Uuid) -> Result<Vector<Relationship>, Box<dyn Error>>;

    fn get_relationship_list_as_table(&self) -> String;
}

impl RelationshipListManagement for Vector<Relationship> {
    type L = Vector<Relationship>;

    fn append_new_relationship(
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

    fn append_related_relationship(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let mut cloned_list = self.clone();
        let new_relationship = Relationship::create_new_related(
            participant_1, participant_2);

        cloned_list.push_back(new_relationship);

        return cloned_list;
    }

    fn append_sequential_relationship(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let mut cloned_list = self.clone();
        let new_relationship = Relationship::create_new_sequential(participant_1, participant_2);

        cloned_list.push_back(new_relationship);

        return cloned_list;
    }

    fn append_parental_relationship(&self, participant_1: Uuid, participant_2: Uuid) -> Self::L {
        let mut cloned_list = self.clone();
        let new_relationship = Relationship::create_new_parental(participant_1, participant_2);

        cloned_list.push_back(new_relationship);

        return cloned_list;
    }

    fn select_relationship_by_id(&self, id: Uuid) -> Result<Relationship, String> {
        let query_result = self.iter().find(|relationship| relationship.get_id() == id)
            .ok_or("cannot find this id within the relationship list".to_string());

        let relationship_clone = query_result?.clone();

        return Ok(relationship_clone);
    }

    fn select_relationship_by_index(&self, index: usize) -> Result<Relationship, Box<dyn Error>> {
        match self.get(index){
            Some(relationship) => {
                return Ok(relationship.clone());
            }
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unable to find Relationship at given Index",
                )));
            }
        }
    }

    fn get_relationship_id(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        let cloned_relationship = self.select_relationship_by_index(index)?;

        Ok(cloned_relationship.get_id())
    }

    fn get_relationship_variant(&self, index: usize) -> Result<RelationshipVariant, Box<dyn Error>> {
        let cloned_relationship = self.select_relationship_by_index(index)?;

        Ok(cloned_relationship.get_variant())
    }

    fn get_relationship_participant_1(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        let relationship_clone = self.select_relationship_by_index(index)?;

        Ok(relationship_clone.get_participant_1())
    }
    fn get_relationship_participant_2(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        let cloned_relationship = self.select_relationship_by_index(index)?;

        Ok(cloned_relationship.get_participant_2())
    }

    fn remove_at_index(&self, index: usize) -> Result<Self::L, Box<dyn Error>> {
        match self.select_relationship_by_index(index){
            Ok(_) => {
                let mut cloned_list = self.clone();

                cloned_list.remove(index);

                return Ok(cloned_list);
            },
            Err(e) => return Err(e)
        }
    }

    fn remove_with_id(&self, id: Uuid) -> Result<Self::L, Box<dyn Error>> {
        let cloned_list = self.clone();
        let target_index = cloned_list.index_of(&cloned_list.select_relationship_by_id(id)?)
            .ok_or("Unable to find relationship with given id")?;

        let updated_list = cloned_list.remove_at_index(target_index)?;

        return Ok(updated_list);
    }

    fn change_relationship_variant(&self, index: usize, variant: &str) -> Result<Self::L, Box<dyn Error>> {
        let updated_relationship = self.select_relationship_by_index(index)?.set_variant(variant)?;

        let updated_list = self.update(index, updated_relationship);

        return Ok(updated_list);
    }

    fn update_relationship_participant_1(&self, index: usize, new_id: Uuid) -> Result<Self::L, Box<dyn Error>> {
        let mut cloned_list = self.clone();
        let cloned_relationship = self.select_relationship_by_index(index)?;

        let updated_relationship = cloned_relationship.set_participant_1(new_id);

        cloned_list.set(index, updated_relationship);

        return Ok(cloned_list);
    }

    fn update_relationship_participant_2(&self, index: usize, new_id: Uuid) -> Result<Self::L, Box<dyn Error>> {
        let mut cloned_list = self.clone();
        let cloned_relationship = self.select_relationship_by_index(index)?;

        let updated_relationship = cloned_relationship.set_participant_2(new_id);
        cloned_list.set(index, updated_relationship);

        Ok(cloned_list)
    }

    fn id_is_present_in_participant_1_list(&self, id: Uuid) -> bool {
        let query_result = self.iter().find(|relationship| relationship.get_participant_1() == id);

        match query_result{
            Some(_) => true,
            None => false
        }
    }

    fn id_is_present_in_participant_2_list(&self, id: Uuid) -> bool {
        let query_result = self.iter().find(|relationship| relationship.get_participant_2() == id);

        match query_result{
            Some(_) => true,
            None => false
        }
    }

    fn id_is_present_in_either_participant_list(&self, id: Uuid) -> bool {
        self.id_is_present_in_participant_1_list(id) 
        || self.id_is_present_in_participant_2_list(id)
    }

    fn get_participant_1_list_for_id(&self, id: Uuid) -> Result<Self::L, Box<dyn Error>> {
        match self.id_is_present_in_participant_1_list(id){
            true => {
                let mut participant_1_list = Vector::new();

                for relationship in self.iter(){
                    if relationship.get_participant_1() == id{
                        participant_1_list.push_back(relationship.clone());
                    }
                }

                return Ok(participant_1_list);
            },
            false => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unable to find Relationship with given Id in participant 1 list",
                )));
            }
        }
    }

    fn get_participant_2_list_for_id(&self, id: Uuid) -> Result<Self::L, Box<dyn Error>> {
        match self.id_is_present_in_participant_2_list(id){
            true => {
                let mut participant_2_list = Vector::new();

                for relationship in self.iter(){
                    if relationship.get_participant_2() == id{
                        participant_2_list.push_back(relationship.clone());
                    }
                }

                return Ok(participant_2_list);
            },
            false => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unable to find Relationship with given Id in participant 2 list",
                )));
            }
        }
    }

    fn get_either_participant_list_for_id(&self, id: Uuid) -> Result<Self::L, Box<dyn Error>> {
        match self.id_is_present_in_either_participant_list(id){
            true => {
                let mut either_participant_list = Vector::new();

                for relationship in self.iter(){
                    if relationship.get_participant_1() == id || relationship.get_participant_2() == id{
                        either_participant_list.push_back(relationship.clone());
                    }
                }

                return Ok(either_participant_list);
            },
            false => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unable to find Relationship with given Id in either participant list",
                )));
            }
        }
    }

    fn get_relationship_list_as_table(&self) -> String {
        Table::new(self).to_string()
        
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_test::{assert_tokens, Configure, Token};
    use item::tests::create_nil_relationship;

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
                Token::Str("id"),
                Token::Str("00000000-0000-0000-0000-000000000000"),
                Token::StructEnd,
                Token::SeqEnd,
            ],
        )
    }
}
