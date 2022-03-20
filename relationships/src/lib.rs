#[allow(dead_code)]
#[derive(PartialEq)]
enum Direction {
    Directed,
    Undirected,
}

#[allow(dead_code)]
struct Relationship {
    direction: Direction,
    participant_1: usize,
    participant_2: usize,
}

impl Relationship {
    #[allow(dead_code)]
    fn create_relationship(
        direction: Direction,
        participant_1: usize,
        participant_2: usize,
    ) -> Self {
        return Relationship {
            direction,
            participant_1,
            participant_2,
        };
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_relationship_creation() {
        let first_participant_id = 1;
        let second_participant_id = 2;
        let direction = Direction::Directed;

        let relationship = Relationship::create_relationship(
            direction,
            first_participant_id,
            second_participant_id,
        );

        assert!(
            relationship.direction == Direction::Directed
                && relationship.participant_1 == 1
                && relationship.participant_2 == 2
        )
    }

    #[test]
    fn two_successful_relationship_creations() {
        let first_participant_id = 1;
        let second_participant_id = 2;
        let third_participant_id = 3;

        let direction_1 = Direction::Directed;
        let direction_2 = Direction::Undirected;

        let relationship_1 = Relationship::create_relationship(
            direction_1,
            first_participant_id,
            second_participant_id,
        );
        let relationship_2 = Relationship::create_relationship(
            direction_2,
            first_participant_id,
            third_participant_id,
        );

        assert!(
            relationship_1.direction == Direction::Directed
                && relationship_1.participant_1 == 1
                && relationship_1.participant_2 == 2
        );
        assert!(
            relationship_2.direction == Direction::Undirected
                && relationship_2.participant_1 == 1
                && relationship_2.participant_2 == 3
        );
    }
}
