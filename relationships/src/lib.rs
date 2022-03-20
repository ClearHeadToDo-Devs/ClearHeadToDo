use uuid::Uuid;

#[allow(dead_code)]
#[derive(PartialEq)]
enum EdgeDirection {
    Directed,
    Undirected,
}

#[allow(dead_code)]
pub struct Relationship {
    id: Uuid,
    direction: EdgeDirection,
    participant_1: Uuid,
    participant_2: Uuid,
}

impl Relationship {
    #[allow(dead_code)]
    fn create_new(
        direction: EdgeDirection,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Self {
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

    fn create_nil_relationship(edge_direction: EdgeDirection, participant_1: Uuid, participant_2: Uuid)->Relationship{
        let nil_id = Uuid::nil();
        return Relationship{
            id: nil_id,
            direction: edge_direction,
            participant_1,
            participant_2,
        }
    }

    #[test]
    fn relationship_id_creation() {
        let nil_id = Uuid::nil();

        let nil_relationship = create_nil_relationship(EdgeDirection::Undirected, nil_id, nil_id);

        assert!(nil_relationship.id==Uuid::nil());
    }

    #[test]
    fn successful_relationship_creation() {
        let first_participant_id = Uuid::new_v4();
        let second_participant_id = Uuid::new_v4();
        let direction = EdgeDirection::Directed;

        let relationship = Relationship::create_new(
            direction,
            first_participant_id,
            second_participant_id,
        );

        assert!(
            relationship.direction == EdgeDirection::Directed
                && relationship.participant_1 == first_participant_id
                && relationship.participant_2 == second_participant_id
        )
    }

    #[test]
    fn two_successful_relationship_creations() {
        let first_participant_id = Uuid::new_v4();
        let second_participant_id = Uuid::new_v4();
        let third_participant_id = Uuid::new_v4();

        let direction_1 = EdgeDirection::Directed;
        let direction_2 = EdgeDirection::Undirected;

        let relationship_1 = Relationship::create_new(
            direction_1,
            first_participant_id,
            second_participant_id,
        );
        let relationship_2 = Relationship::create_new(
            direction_2,
            first_participant_id,
            third_participant_id,
        );

        assert!(
            relationship_1.direction == EdgeDirection::Directed
                && relationship_1.participant_1 == first_participant_id
                && relationship_1.participant_2 == second_participant_id
        );
        assert!(
            relationship_2.direction == EdgeDirection::Undirected
                && relationship_2.participant_1 == first_participant_id
                && relationship_2.participant_2 == third_participant_id
        );
    }
}
