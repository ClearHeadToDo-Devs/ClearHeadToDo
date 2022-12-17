use crate::relationship::item::RelationshipVariant;
use crate::relationship::Relationship;
use im::Vector;

use std::error::Error;
use uuid::Uuid;

pub trait RelationshipListManagement {
    type L: RelationshipListManagement;
    fn append_new_relationship(
        &self,
        target_variant: &str,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Result<Self::L, Box<dyn Error>>;

    fn select_relationship_by_id(&self, id: Uuid) -> Result<Relationship, String>;
    fn select_relationship_by_index(&self, index: usize) -> Result<Relationship, Box<dyn Error>>;


    fn remove_at_index(&self, index: usize) -> Result<Self::L, Box<dyn Error>>;
    fn remove_with_id(&self, id: Uuid) -> Result<Self::L, Box<dyn Error>>;

    fn change_relationship_variant(&self, index: usize, variant: &str) -> Result<Self::L, Box<dyn Error>>;
    fn update_relationship_participant_1(&self, index: usize, new_id: Uuid) -> Result<Self::L, Box<dyn Error>>;
    fn update_relationship_participant_2(&self, index: usize, new_id: Uuid) -> Result<Self::L, Box<dyn Error>>;

}

pub trait RelationshipListViewer{
    type L: RelationshipListManagement;
    fn get_relationship_id(&self, index: usize) -> Result<Uuid, Box<dyn Error>>;
    fn get_relationship_variant(&self, index: usize) -> Result<RelationshipVariant, Box<dyn Error>>;
    fn get_relationship_participant_1(&self, index: usize) -> Result<Uuid, Box<dyn Error>>;
    fn get_relationship_participant_2(&self, index: usize) -> Result<Uuid, Box<dyn Error>>;

    fn id_is_present_in_participant_1_list(&self, id: Uuid) -> bool;
    fn id_is_present_in_participant_2_list(&self, id: Uuid) -> bool;
    fn id_is_present_in_either_participant_list(&self, id: Uuid) -> bool;

    fn filter_by_participants(&self, list: String, id: Uuid) -> Result<Self::L, Box<dyn Error>>;

    fn get_participant_1_list_for_id(&self, id: Uuid) -> Result<Vector<Relationship>, Box<dyn Error>>;
    fn get_participant_2_list_for_id(&self, id: Uuid) -> Result<Vector<Relationship>, Box<dyn Error>>;
    fn get_either_participant_list_for_id(&self, id: Uuid) -> Result<Vector<Relationship>, Box<dyn Error>>;

    fn filter_by_variant(&self, variant: &str) -> Result<Vector<Relationship>, Box<dyn Error>>;

    fn get_children_for_id(&self, id: Uuid) -> Result<Vector<Uuid>, Box<dyn Error>>;

    fn get_relationship_list_as_table(&self) -> String;

}

