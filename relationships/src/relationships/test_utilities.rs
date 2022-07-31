use crate::Relationship;
use crate::RelationshipVariant;
use crate::Uuid;
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
