pub mod relationship_variants;
pub use relationship_variants::*;

use uuid::Uuid;

use im::Vector;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct Relationship {
    id: Uuid,
    variant: RelationshipVariant,
    participant_1: Uuid,
    participant_2: Uuid,
}

trait RelationshipManagement {
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
    fn base_relationship_creation() {
        let nil_relationship =
            Relationship::create_new("related", Uuid::new_v4(), Uuid::new_v4()).unwrap();

        assert!(
            nil_relationship.variant == RelationshipVariant::Related(EdgeDirection::Undirected)
        );
    }

    #[test]
    fn relationship_id_creation() {
        let nil_relationship = create_nil_relationship(
            RelationshipVariant::create_related(),
            Uuid::nil(),
            Uuid::nil(),
        );

        assert!(nil_relationship.id == Uuid::nil());
    }

    #[test]
    fn unique_relationship_participants() {
        let relationship =
            Relationship::create_new("related", Uuid::new_v4(), Uuid::new_v4()).unwrap();

        assert!(relationship.participant_2 != relationship.participant_1)
    }

    #[test]
    fn unique_relationship_id() {
        let nil_participant_id = Uuid::nil();

        let relationship_1 =
            Relationship::create_new("related", nil_participant_id, nil_participant_id).unwrap();
        let relationship_2 =
            Relationship::create_new("related", nil_participant_id, nil_participant_id).unwrap();

        assert!(relationship_2.id != relationship_1.id);
    }

    #[test]
    fn invalid_relationship_variant_input() {
        let invalid_relationship =
            Relationship::create_new("bad variant", Uuid::new_v4(), Uuid::new_v4()).unwrap_err();

        assert!(invalid_relationship == "invalid relationship variant");
    }

    #[test]
    fn create_related_relationship() {
        let nil_participant_id = Uuid::nil();

        let new_related_relationship =
            Relationship::create_new_related(nil_participant_id, nil_participant_id);

        assert!(
            new_related_relationship.variant
                == RelationshipVariant::Related(EdgeDirection::Undirected)
        )
    }

    #[test]
    fn create_subsequent() {
        let nil_participant_id = Uuid::nil();

        let new_sequential_relationship =
            Relationship::create_new_sequential(nil_participant_id, nil_participant_id);

        assert!(
            new_sequential_relationship.variant
                == RelationshipVariant::Sequential(EdgeDirection::Directed)
        );
    }

    #[test]
    fn create_parental() {
        let nil_particpant_id = Uuid::nil();

        let new_parental_relationship =
            Relationship::create_new_parental(nil_particpant_id, nil_particpant_id);

        assert!(
            new_parental_relationship.variant
                == RelationshipVariant::Parental(EdgeDirection::Directed)
        )
    }
}
