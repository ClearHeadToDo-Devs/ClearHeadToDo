pub mod relationship_variants;
pub use relationship_variants::*;

use crate::Uuid;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Relationship {
    id: Uuid,
    variant: RelationshipVariant,
    participant_1: Uuid,
    participant_2: Uuid,
}

pub trait RelationshipManagement {
    type R: RelationshipManagement;
    type V: RelationshipVariantManagement;
    fn create_new(
        variant_str: &str,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Result<Self::R, Box<dyn Error>>;
    fn create_new_related(participant_1: Uuid, participant_2: Uuid) -> Self::R;
    fn create_new_sequential(participant_1: Uuid, participant_2: Uuid) -> Self::R;
    fn create_new_parental(participant_1: Uuid, participant_2: Uuid) -> Self::R;

    fn get_id(&self) -> Uuid;
    fn get_variant(&self) -> Self::V;
    fn get_participant_1(&self) -> Uuid;
    fn get_participant_2(&self) -> Uuid;
    fn get_edge_direction(&self) -> String;

    fn set_variant(&self, target_variant: &str) -> Result<Self::R, String>;
    fn set_participant_1(&self, new_id: Uuid) -> Self::R;
    fn set_participant_2(&self, new_id: Uuid) -> Self::R;
}

impl RelationshipManagement for Relationship {
    type R = Relationship;
    type V = RelationshipVariant;

    fn create_new(
        variant_str: &str,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Result<Self, Box<dyn Error>> {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::create_from_string(&variant_str)?;

        Ok(Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        })
    }
    fn create_new_related(participant_1: Uuid, participant_2: Uuid) -> Self {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::create_related();

        Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        }
    }

    fn create_new_sequential(participant_1: Uuid, participant_2: Uuid) -> Self {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::create_sequential();

        Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        }
    }

    fn create_new_parental(participant_1: Uuid, participant_2: Uuid) -> Self {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::create_parental();

        Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        }
    }

    fn get_id(&self) -> Uuid {
        return self.id;
    }

    fn get_variant(&self) -> Self::V {
        return self.variant;
    }

    fn get_edge_direction(&self) -> String {
        return self.variant.get_edge_direction();
    }

    fn get_participant_1(&self) -> Uuid {
        return self.participant_1;
    }

    fn get_participant_2(&self) -> Uuid {
        return self.participant_2;
    }

    fn set_variant(&self, target_variant: &str) -> Result<Relationship, String> {
        let variant = RelationshipVariant::create_from_string(target_variant)?;
        let mut cloned_relationship = self.clone();

        cloned_relationship.variant = variant;

        Ok(cloned_relationship)
    }

    fn set_participant_1(&self, new_id: Uuid) -> Self::R {
        let mut cloned_relationship = self.clone();

        cloned_relationship.participant_1 = new_id;

        return cloned_relationship;
    }

    fn set_participant_2(&self, new_id: Uuid) -> Self::R {
        let mut cloned_relationship = self.clone();

        cloned_relationship.participant_2 = new_id;

        return cloned_relationship;
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use relationship_variants::edge_direction::EdgeDirectionality;
    use serde_test::{assert_tokens, Configure, Token};

    pub fn create_nil_relationship(
        variant: RelationshipVariant,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Relationship {
        let id = Uuid::nil();
        return Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        };
    }

    #[test]
    fn id_creation() {
        let nil_relationship = create_nil_relationship(
            RelationshipVariant::Related(EdgeDirectionality::Undirected),
            Uuid::nil(),
            Uuid::nil(),
        );

        assert!(nil_relationship.id.is_nil());
    }

    #[test]
    fn ensure_unique_id() {
        let variant_string = "related".to_string();
        let nil_uuid_string = Uuid::nil();

        let relationship_1 =
            Relationship::create_new(&variant_string, nil_uuid_string, nil_uuid_string).unwrap();
        let relationship_2 =
            Relationship::create_new(&variant_string, nil_uuid_string, nil_uuid_string).unwrap();

        assert!(relationship_2.id != relationship_1.id);
    }

    #[test]
    fn invalid_relationship_variant_input() {
        let nil_uuid_string = Uuid::nil();
        let variant_string = "bad bariant".to_string();

        let invalid_relationship =
            Relationship::create_new(&variant_string, nil_uuid_string, nil_uuid_string)
                .unwrap_err()
                .to_string();

        assert!(invalid_relationship == "invalid relationship variant");
    }

    #[test]
    fn create_related() {
        let new_related_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        assert!(
            new_related_relationship.variant
                == RelationshipVariant::Related(EdgeDirectionality::Undirected)
        )
    }

    #[test]
    fn create_subsequent() {
        let new_sequential_relationship =
            Relationship::create_new_sequential(Uuid::nil(), Uuid::nil());

        assert!(
            new_sequential_relationship.variant
                == RelationshipVariant::Sequential(EdgeDirectionality::Directed)
        );
    }

    #[test]
    fn create_parental() {
        let new_parental_relationship = Relationship::create_new_parental(Uuid::nil(), Uuid::nil());

        assert!(
            new_parental_relationship.variant
                == RelationshipVariant::Parental(EdgeDirectionality::Directed)
        )
    }

    #[test]
    fn get_related_variant() {
        let example_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        let variant = example_relationship.get_variant();

        assert!(variant == RelationshipVariant::Related(EdgeDirectionality::Undirected));
    }

    #[test]
    fn get_parental() {
        let new_related_relationship = Relationship::create_new_parental(Uuid::nil(), Uuid::nil());

        let variant = new_related_relationship.get_variant();

        assert!(variant == RelationshipVariant::Parental(EdgeDirectionality::Directed));
    }

    #[test]
    fn get_sequential() {
        let new_related_relationship =
            Relationship::create_new_sequential(Uuid::nil(), Uuid::nil());

        let variant = new_related_relationship.get_variant();

        assert!(variant == RelationshipVariant::Sequential(EdgeDirectionality::Directed))
    }

    #[test]
    fn get_edge_direction_from_rel() {
        let test_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        let edge_direction = test_relationship.get_edge_direction();

        assert!(edge_direction == "Undirected")
    }

    #[test]
    fn get_id() {
        let test_relationship = create_nil_relationship(
            RelationshipVariant::create_related(),
            Uuid::nil(),
            Uuid::nil(),
        );

        let id = test_relationship.get_id();

        assert!(id.is_nil())
    }

    #[test]
    fn get_particpant_1_id() {
        let test_relationship = Relationship::create_new_related(Uuid::new_v4(), Uuid::nil());

        let participant_1 = test_relationship.get_participant_1();

        assert!(participant_1 == test_relationship.participant_1)
    }

    #[test]
    fn get_participant_2_id() {
        let test_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::new_v4());

        let participant_2 = test_relationship.get_participant_2();

        assert!(participant_2 == test_relationship.participant_2)
    }

    #[test]
    fn get_edge_direction() {
        let test_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        let edge_direction = test_relationship.get_edge_direction();

        assert!(edge_direction == "Undirected")
    }

    #[test]
    fn change_relationship_variant() {
        let test_relationship = Relationship::create_new_sequential(Uuid::nil(), Uuid::nil());

        let updated_relationship = test_relationship.set_variant("parental").unwrap();

        assert!(
            updated_relationship
                == Relationship {
                    variant: RelationshipVariant::Parental(EdgeDirectionality::Directed),
                    ..test_relationship
                }
        )
    }

    #[test]
    fn change_undirected_to_directed() {
        let test_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        let updated_relationship = test_relationship.set_variant("parental").unwrap();

        assert!(
            updated_relationship
                == Relationship {
                    variant: RelationshipVariant::Parental(EdgeDirectionality::Directed),
                    ..test_relationship
                }
        )
    }

    #[test]
    fn set_participant_1() {
        let test_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());
        let new_uuid = Uuid::new_v4();

        let updated_relationship = test_relationship.set_participant_1(new_uuid);

        assert!(updated_relationship.participant_1 == new_uuid)
    }

    #[test]
    fn set_participant_2() {
        let test_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());
        let new_uuid = Uuid::new_v4();

        let updated_relationship = test_relationship.set_participant_2(new_uuid);

        assert!(updated_relationship.participant_2 == new_uuid)
    }

    #[test]
    fn serialization_and_deserialization() {
        let example_relationship = create_nil_relationship(
            RelationshipVariant::create_related(),
            Uuid::nil(),
            Uuid::nil(),
        );

        assert_tokens(
            &example_relationship.readable(),
            &[
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
            ],
        )
    }
}
