pub mod variants;
pub use variants::*;

use crate::Uuid;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::str::FromStr;
use tabled::Tabled;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Tabled)]
pub struct Relationship {
    variant: RelationshipVariant,
    participant_1: Uuid,
    participant_2: Uuid,
    #[tabled(skip)]
    id: Uuid,
}

impl Relationship {
    pub fn create_new(
        variant_str: &str,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Result<Self, Box<dyn Error>> {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::from_str(&variant_str)?;

        Ok(Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        })
    }
    pub fn create_new_related(participant_1: Uuid, participant_2: Uuid) -> Self {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::create_related();

        Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        }
    }

    pub fn create_new_sequential(participant_1: Uuid, participant_2: Uuid) -> Self {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::create_sequential();

        Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        }
    }

    pub fn create_new_parental(participant_1: Uuid, participant_2: Uuid) -> Self {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::create_parental();

        Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        }
    }

    pub fn get_id(&self) -> Uuid {
        return self.id;
    }

    pub fn get_variant(&self) -> RelationshipVariant {
        return self.variant;
    }

    pub fn get_edge_direction(&self) -> String {
        return self.variant.get_edge_direction();
    }

    pub fn get_participant_1(&self) -> Uuid {
        return self.participant_1;
    }

    pub fn get_participant_2(&self) -> Uuid {
        return self.participant_2;
    }

    pub fn set_variant(&self, target_variant: &str) -> Result<Relationship, String> {
        let variant = RelationshipVariant::from_str(target_variant)?;
        let mut cloned_relationship = self.clone();

        cloned_relationship.variant = variant;

        Ok(cloned_relationship)
    }

    pub fn set_participant_1(&self, new_id: Uuid) -> Relationship {
        let mut cloned_relationship = self.clone();

        cloned_relationship.participant_1 = new_id;

        return cloned_relationship;
    }

    pub fn set_participant_2(&self, new_id: Uuid) -> Relationship {
        let mut cloned_relationship = self.clone();

        cloned_relationship.participant_2 = new_id;

        return cloned_relationship;
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use serde_test::{assert_tokens, Configure, Token};

    pub fn create_relationship_with_nil_participants(variant: RelationshipVariant) -> Relationship {
        Relationship {
            id: Uuid::new_v4(),
            variant,
            participant_1: Uuid::nil(),
            participant_2: Uuid::nil(),
        }
    }

    pub fn create_nil_relationship(
    ) -> Relationship {
        let id = Uuid::nil();

        return Relationship {
            id,
            variant: RelationshipVariant::create_related(),
            participant_1: Uuid::nil(),
            participant_2: Uuid::nil(),
        };
    }

    fn create_related_with_double_nil() -> Relationship {
        Relationship::create_new_related(Uuid::nil(), Uuid::nil())
    }
    

    #[test]
    fn get_related_variant() {
        let example_relationship = create_relationship_with_nil_participants(
            RelationshipVariant::create_related());

        let variant = example_relationship.get_variant();

        assert_eq!(variant , RelationshipVariant::create_related());
    }

    #[test]
    fn get_parental() {
        let new_related_relationship = create_relationship_with_nil_participants(
            RelationshipVariant::create_parental());

        let variant = new_related_relationship.get_variant();

        assert_eq!(variant , RelationshipVariant::create_parental());
    }

    #[test]
    fn get_sequential() {
        let new_related_relationship = create_relationship_with_nil_participants(
            RelationshipVariant::create_sequential());

        let variant = new_related_relationship.get_variant();

        assert_eq!(variant , RelationshipVariant::create_sequential());
    }

    #[test]
    fn get_edge_direction_from_rel() {
        let test_relationship = create_related_with_double_nil();

        let edge_direction = test_relationship.get_edge_direction();

        assert_eq!(edge_direction , "Undirected")
    }

    #[test]
    fn get_id() {
        let test_relationship = create_nil_relationship();

        let id = test_relationship.get_id();

        assert!(id.is_nil())
    }

    #[test]
    fn get_particpant_1_id() {
        let test_relationship = create_related_with_double_nil();

        let participant_1 = test_relationship.get_participant_1();

        assert_eq!(participant_1 , test_relationship.participant_1)
    }

    #[test]
    fn get_participant_2_id() {
        let test_relationship = create_related_with_double_nil();

        let participant_2 = test_relationship.get_participant_2();

        assert_eq!(participant_2 , test_relationship.participant_2)
    }

    #[test]
    fn get_edge_direction() {
        let test_relationship = create_related_with_double_nil();

        let edge_direction = test_relationship.get_edge_direction();

        assert_eq!(edge_direction , "Undirected")
    }


    #[test]
    fn ensure_unique_id() {
        let relationship_1 = create_related_with_double_nil();
        let relationship_2 = create_related_with_double_nil();

        assert!(relationship_2.get_id() != relationship_1.get_id());
    }

    #[test]
    fn invalid_relationship_variant_input() {
        let bad_variant = Relationship::create_new("bad_variant", Uuid::nil(), Uuid::nil());

        let error = bad_variant.unwrap_err();

        assert_eq!(error.to_string() , "invalid relationship variant");
    }

    #[test]
    fn create_related() {
        let new_related_relationship = create_related_with_double_nil();

        assert_eq!(new_related_relationship.get_variant(), RelationshipVariant::create_related()
        )
    }

    #[test]
    fn create_subsequent() {
        let new_sequential_relationship = create_relationship_with_nil_participants(
            RelationshipVariant::create_sequential());

        assert_eq!(new_sequential_relationship.get_variant(),RelationshipVariant::create_sequential());
    }

    #[test]
    fn create_parental() {
        let new_parental_relationship = create_relationship_with_nil_participants(
            RelationshipVariant::create_parental());

        assert_eq!(new_parental_relationship.get_variant() , RelationshipVariant::create_parental());
    }


    #[test]
    fn change_relationship_variant() {
        let test_relationship = create_related_with_double_nil();

        let updated_relationship = test_relationship.set_variant("parental").unwrap();

        assert!(updated_relationship.get_variant() == RelationshipVariant::create_parental())
    }

    #[test]
    fn change_undirected_to_directed() {
        let test_relationship = create_related_with_double_nil();

        let updated_relationship = test_relationship.set_variant("parental").unwrap();

        assert!(updated_relationship.get_variant() == RelationshipVariant::create_parental())
    }

    #[test]
    fn set_participant_1() {
        let test_relationship = create_related_with_double_nil();
        let new_uuid = Uuid::new_v4();

        let updated_relationship = test_relationship.set_participant_1(new_uuid);

        assert_eq!(updated_relationship.get_participant_1() , new_uuid)
    }

    #[test]
    fn set_participant_2() {
        let test_relationship = create_related_with_double_nil();
        let new_uuid = Uuid::new_v4();

        let updated_relationship = test_relationship.set_participant_2(new_uuid);

        assert_eq!(updated_relationship.participant_2 , new_uuid)
    }

    #[test]
    fn serialization_and_deserialization() {
        let example_relationship = create_nil_relationship();

        assert_tokens(
            &example_relationship.readable(),
            &[
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
            ],
        )
    }
}
