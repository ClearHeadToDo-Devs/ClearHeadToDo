#[derive(PartialEq)]
enum RelationshipType {
    ParentChild,
    PreviousSubsequent,
    Related,
}
struct Relationship {
    relationship_type: RelationshipType,
    participant_1: usize,
    participant_2: usize
}

impl Relationship{
    #[allow(dead_code)]
    fn create_relationship(relationship_type: RelationshipType, participant_1: usize, participant_2: usize) -> Self{
        return Relationship{
            relationship_type: relationship_type,
            participant_1, 
            participant_2
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_relationship_creation() {
        let first_participant_id = 1;
        let second_participant_id = 2;
        let relationship_type = RelationshipType::ParentChild;

        let relationship = Relationship::create_relationship(relationship_type, first_participant_id, second_participant_id);

        assert!(relationship.relationship_type==RelationshipType::ParentChild && relationship.participant_1==1 && relationship.participant_2==2)
    }

    #[test]
    fn two_successful_relationship_creations(){
        let first_participant_id = 1;
        let second_participant_id = 2;
        let third_participant_id = 3;

        let relationship_type_1 = RelationshipType::ParentChild;
        let relationship_type_2 = RelationshipType::PreviousSubsequent;

        let relationship_1 = Relationship::create_relationship(relationship_type_1, first_participant_id, second_participant_id);
        let relationship_2 = Relationship::create_relationship(relationship_type_2, first_participant_id, third_participant_id);

        assert!(relationship_1.relationship_type==RelationshipType::ParentChild && relationship_1.participant_1==1 && relationship_1.participant_2==2);
        assert!(relationship_2.relationship_type==RelationshipType::PreviousSubsequent && relationship_2.participant_1==1 && relationship_2.participant_2==3);
    }
}
