use uuid::Uuid;

pub struct Relationship {
    id: Uuid,
    target: Uuid,
    source: Uuid,
}

impl Relationship {
    fn new(id: Uuid, target: Uuid, source: Uuid) -> Self {
        Self { id, target, source }
    }
}

#[derive(PartialEq)]
enum RelationshipType {
    Parental = 1,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_minimal_relationship() {
        let relationship = Relationship::new(Uuid::nil(), Uuid::nil(), Uuid::nil());

        assert!(relationship.id.is_nil());
        assert!(relationship.target.is_nil());
        assert!(relationship.source.is_nil())
    }

    #[test]
    fn create_parental_type() {
        let relationship_type = RelationshipType::Parental;

        assert!(relationship_type as usize == 1 )
    }
}
