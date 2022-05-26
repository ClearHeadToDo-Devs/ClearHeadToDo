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

    fn get_id_string(&self) -> String;
    fn get_variant_string(&self) -> String;
    fn get_edge_direction_as_string(&self) -> String;
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

    fn get_id_string(&self) -> String {
        return self.id.to_string();
    }

    fn get_variant_string(&self) -> String {
        return format!("{}", self.variant).to_string();
    }

    fn get_edge_direction_as_string(&self) -> String {
        return self.variant.get_edge_direction_as_string();
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

        assert!(nil_relationship.id == Uuid::nil());
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
    fn return_variant_string() {
        let example_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        let variant_string = example_relationship.get_variant_string();

        assert!(variant_string == "Related: Undirected".to_string())
    }

    #[test]
    fn export_related_variant_string() {
        let new_related_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        let variant_string = new_related_relationship.get_variant_string();

        assert!(variant_string == "Related: Undirected")
    }

    #[test]
    fn export_parental_variant_string() {
        let new_related_relationship = Relationship::create_new_parental(Uuid::nil(), Uuid::nil());

        let variant_string = new_related_relationship.get_variant_string();

        assert!(variant_string == "Parental: Directed")
    }

    #[test]
    fn export_sequential_variant_string() {
        let new_related_relationship =
            Relationship::create_new_sequential(Uuid::nil(), Uuid::nil());

        let variant_string = new_related_relationship.get_variant_string();

        assert!(variant_string == "Sequential: Directed")
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
    fn cahnge_undirected_to_directed() {
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
    fn get_edge_direction_string() {
        let test_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        let edge_direction = test_relationship.get_edge_direction_as_string();

        assert!(edge_direction == "Undirected")
    }

    #[test]
    fn get_id_string() {
        let test_relationship = create_nil_relationship(
            RelationshipVariant::create_related(),
            Uuid::nil(),
            Uuid::nil(),
        );

        let id_string = test_relationship.get_id_string();

        assert!(id_string == Uuid::nil().to_string())
    }
}
