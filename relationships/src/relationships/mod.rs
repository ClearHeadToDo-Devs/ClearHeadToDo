pub mod relationship_variants;
pub use relationship_variants::*;

use crate::Uuid;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub struct Relationship {
    id: Uuid,
    variant: RelationshipVariant,
    participant_1: Uuid,
    participant_2: Uuid,
}

pub trait RelationshipManagement {
    type R: RelationshipManagement;
    type V: RelationshipVariantManagement;
    #[allow(dead_code)]
    fn create_new(
        variant_str: &str,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Result<Self::R, String>;
    fn create_new_related(participant_1: Uuid, participant_2: Uuid) -> Self::R;
    fn create_new_sequential(participant_1: Uuid, participant_2: Uuid) -> Self::R;
    fn create_new_parental(participant_1: Uuid, participant_2: Uuid) -> Self::R;

    fn change_variant(&self, target_variant: &str) -> Result<Self::R, String>;
    fn change_edge_direction(&self) -> Self::R;

    fn get_id(&self) -> Uuid;
    fn get_participant_1(&self) -> Uuid;
    fn get_variant(&self) -> Self::V;
    fn get_edge_direction(&self) -> EdgeDirection;
}

impl RelationshipManagement for Relationship {
    type R = Relationship;
    type V = RelationshipVariant;

    fn create_new(
        variant_str: &str,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Result<Self, String> {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::create_from_string(variant_str)?;
        return Ok(Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        });
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

    fn change_variant(&self, target_variant: &str) -> Result<Self, String> {
        return Ok(Relationship {
            variant: RelationshipVariant::create_from_string(target_variant)?,
            ..self.to_owned()
        });
    }

    fn change_edge_direction(&self) -> Self {
        return Relationship {
            variant: self.variant.change_edge_direction(),
            ..self.to_owned()
        };
    }

    fn get_id(&self) -> Uuid {
        return self.id;
    }

    fn get_participant_1(&self) -> Uuid {
        return self.participant_1;
    }

    fn get_variant(&self) -> RelationshipVariant {
        return self.variant;
    }

    fn get_edge_direction(&self) -> EdgeDirection {
        return self.variant.get_edge_direction();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn create_nil_relationship(
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
    fn relationship_id_creation() {
        let nil_relationship = create_nil_relationship(
            RelationshipVariant::Related(EdgeDirection::Undirected),
            Uuid::nil(),
            Uuid::nil(),
        );

        assert!(nil_relationship.id.is_nil());
    }

    #[test]
    fn unique_relationship_id() {
        let relationship_1 = Relationship::create_new("related", Uuid::nil(), Uuid::nil()).unwrap();
        let relationship_2 = Relationship::create_new("related", Uuid::nil(), Uuid::nil()).unwrap();

        assert!(relationship_2.id != relationship_1.id);
    }

    #[test]
    fn invalid_relationship_variant_input() {
        let invalid_relationship =
            Relationship::create_new("bad variant", Uuid::nil(), Uuid::nil()).unwrap_err();

        assert!(invalid_relationship == "invalid relationship variant");
    }

    #[test]
    fn create_related_relationship() {
        let new_related_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        assert!(
            new_related_relationship.variant
                == RelationshipVariant::Related(EdgeDirection::Undirected)
        )
    }

    #[test]
    fn create_subsequent() {
        let new_sequential_relationship =
            Relationship::create_new_sequential(Uuid::nil(), Uuid::nil());

        assert!(
            new_sequential_relationship.variant
                == RelationshipVariant::Sequential(EdgeDirection::Directed)
        );
    }

    #[test]
    fn create_parental() {
        let new_parental_relationship = Relationship::create_new_parental(Uuid::nil(), Uuid::nil());

        assert!(
            new_parental_relationship.variant
                == RelationshipVariant::Parental(EdgeDirection::Directed)
        )
    }

    #[test]
    fn get_variant() {
        let example_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        let variant = example_relationship.get_variant();

        assert!(variant == RelationshipVariant::Related(EdgeDirection::Undirected))
    }

    #[test]
    fn export_parental_variant_string() {
        let new_related_relationship = Relationship::create_new_parental(Uuid::nil(), Uuid::nil());

        let variant = new_related_relationship.get_variant();

        assert!(variant == RelationshipVariant::Parental(EdgeDirection::Directed))
    }

    #[test]
    fn export_sequential_variant_string() {
        let new_related_relationship =
            Relationship::create_new_sequential(Uuid::nil(), Uuid::nil());

        let variant = new_related_relationship.get_variant();

        assert!(variant == RelationshipVariant::Sequential(EdgeDirection::Directed))
    }

    #[test]
    fn change_relationship_variant() {
        let test_relationship = Relationship::create_new_sequential(Uuid::nil(), Uuid::nil());

        let updated_relationship = test_relationship.change_variant("parental").unwrap();

        assert!(
            updated_relationship
                == Relationship {
                    variant: RelationshipVariant::Parental(EdgeDirection::Directed),
                    ..test_relationship
                }
        )
    }

    #[test]
    fn change_undirected_to_directed() {
        let test_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        let updated_relationship = test_relationship.change_variant("parental").unwrap();

        assert!(
            updated_relationship
                == Relationship {
                    variant: RelationshipVariant::Parental(EdgeDirection::Directed),
                    ..test_relationship
                }
        )
    }

    #[test]
    fn change_variant_direction() {
        let test_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        let updated_relationship = test_relationship.change_edge_direction();

        assert!(
            updated_relationship
                == Relationship {
                    variant: RelationshipVariant::Related(EdgeDirection::Directed),
                    ..test_relationship
                }
        )
    }

    #[test]
    fn get_edge_direction_from_rel() {
        let test_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        let edge_direction = test_relationship.get_edge_direction();

        assert!(edge_direction == EdgeDirection::Undirected)
    }

    #[test]
    fn get_id_string() {
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
        let test_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::new_v4());

        let participant_1 = test_relationship.get_participant_1();

        assert!(participant_1.is_nil())
    }

    #[test]
    fn get_edge_direction() {
        let test_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        //let edge_direction = test_relationship.get_variant();
    }
}
