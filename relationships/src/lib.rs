use uuid::Uuid;

#[allow(dead_code)]
pub struct Relationship {
    id: Uuid,
    direction: EdgeDirection,
    participant_1: Uuid,
    participant_2: Uuid,
}

#[allow(dead_code)]
#[derive(PartialEq)]
enum EdgeDirection {
    Directed,
    Undirected,
}

trait RelationshipManagement {
    #[allow(dead_code)]
    fn create_new(direction: EdgeDirection, participant_1: Uuid, participant_2: Uuid) -> Self;
}

impl RelationshipManagement for Relationship {
    fn create_new(direction: EdgeDirection, participant_1: Uuid, participant_2: Uuid) -> Self {
        let relationship_id = Uuid::new_v4();
        return Relationship {
            id: relationship_id,
            direction,
            participant_1,
            participant_2,
        };
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn create_nil_relationship(
        edge_direction: EdgeDirection,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Relationship {
        let nil_id = Uuid::nil();
        return Relationship {
            id: nil_id,
            direction: edge_direction,
            participant_1,
            participant_2,
        };
    }

    #[test]
    fn relationship_id_creation() {
        let nil_id = Uuid::nil();

        let nil_relationship = create_nil_relationship(EdgeDirection::Undirected, nil_id, nil_id);

        assert!(nil_relationship.id == Uuid::nil());
    }

    #[test]
    fn undirected_relationship_creation() {
        let nil_participant_id = Uuid::nil();

        let nil_relationship = Relationship::create_new(
            EdgeDirection::Undirected,
            nil_participant_id,
            nil_participant_id,
        );

        assert!(nil_relationship.direction == EdgeDirection::Undirected);
    }

    #[test]
    fn directed_relationship_creation() {
        let nil_participant_id = Uuid::nil();

        let nil_relationship = Relationship::create_new(
            EdgeDirection::Directed,
            nil_participant_id,
            nil_participant_id,
        );

        assert!(nil_relationship.direction == EdgeDirection::Directed);
    }

    #[test]
    fn unique_relationship_participants() {
        let first_participant_id = Uuid::new_v4();
        let second_participant_id = Uuid::new_v4();
        let direction = EdgeDirection::Directed;

        let relationship =
            Relationship::create_new(direction, first_participant_id, second_participant_id);

        assert!(relationship.participant_2 != relationship.participant_1)
    }

    #[test]
    fn unique_relationship_id() {
        let nil_participant_id = Uuid::nil();

        let relationship_1 = Relationship::create_new(
            EdgeDirection::Directed,
            nil_participant_id,
            nil_participant_id,
        );
        let relationship_2 = Relationship::create_new(
            EdgeDirection::Directed,
            nil_participant_id,
            nil_participant_id,
        );

        assert!(relationship_2.id != relationship_1.id);
    }
}
