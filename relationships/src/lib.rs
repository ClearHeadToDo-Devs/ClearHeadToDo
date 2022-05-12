pub mod relationship_variants;
pub use relationship_variants::*;

use uuid::Uuid;

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

    fn create_new_related(participant_1: Uuid, participant_2: Uuid) -> Result<Self::R, String>;
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
        let variant = RelationshipVariant::create_variant_from_string(variant_str)?;
        return Ok(Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        });
    }
    fn create_new_related(participant_1: Uuid, participant_2: Uuid) -> Result<Self, String> {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::create_related_variant();
        return Ok(Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        });
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
    fn undirected_relationship_creation() {
        let nil_participant_id = Uuid::nil();

        let nil_relationship =
            Relationship::create_new("related", nil_participant_id, nil_participant_id).unwrap();

        assert!(nil_relationship.variant == RelationshipVariant::create_related_variant());
    }

    #[test]
    fn relationship_id_creation() {
        let nil_id = Uuid::nil();

        let nil_relationship = create_nil_relationship(
            RelationshipVariant::create_related_variant(),
            nil_id,
            nil_id,
        );

        assert!(nil_relationship.id == Uuid::nil());
    }

    #[test]
    fn unique_relationship_participants() {
        let first_participant_id = Uuid::new_v4();
        let second_participant_id = Uuid::new_v4();
        let variant_str = "related";

        let relationship =
            Relationship::create_new(variant_str, first_participant_id, second_participant_id)
                .unwrap();

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
        let nil_participant_id = Uuid::nil();

        let invalid_relationship =
            Relationship::create_new("bad variant", nil_participant_id, nil_participant_id)
                .unwrap_err();

        assert!(invalid_relationship == "invalid relationship variant");
    }

    #[test]
    fn create_related_relationship() {
        let nil_participant_id = Uuid::nil();

        let new_related_relationship =
            Relationship::create_new_related(nil_participant_id, nil_participant_id).unwrap();

        assert!(
            new_related_relationship.variant
                == RelationshipVariant::Related(EdgeDirection::Undirected)
        )
    }
}
