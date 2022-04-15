pub mod edge_direction;
pub use edge_direction::*;

pub mod relationship_variants;
pub use relationship_variants::*;

use uuid::Uuid;

#[allow(dead_code)]
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
    fn create_new(variant: Self::V, participant_1: Uuid, participant_2: Uuid) -> Self::R;
}

impl RelationshipManagement for Relationship {
    type R = Relationship;
    type V = RelationshipVariant;

    fn create_new(variant: Self::V, participant_1: Uuid, participant_2: Uuid) -> Self {
        let id = Uuid::new_v4();
        return Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        };
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

        let nil_relationship = Relationship::create_new(
            RelationshipVariant::create_related_variant(),
            nil_participant_id,
            nil_participant_id,
        );

        assert!(
            nil_relationship.variant == RelationshipVariant::Related(EdgeDirection::Undirected)
        );
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
        let direction = RelationshipVariant::create_related_variant();

        let relationship =
            Relationship::create_new(direction, first_participant_id, second_participant_id);

        assert!(relationship.participant_2 != relationship.participant_1)
    }

    #[test]
    fn unique_relationship_id() {
        let nil_participant_id = Uuid::nil();

        let relationship_1 = Relationship::create_new(
            RelationshipVariant::create_related_variant(),
            nil_participant_id,
            nil_participant_id,
        );
        let relationship_2 = Relationship::create_new(
            RelationshipVariant::create_related_variant(),
            nil_participant_id,
            nil_participant_id,
        );

        assert!(relationship_2.id != relationship_1.id);
    }
}
