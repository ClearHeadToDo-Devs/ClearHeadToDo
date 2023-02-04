use uuid::Uuid;

pub struct Relationship {
    id: Uuid,
    variant: RelationshipVariant,
    target: Uuid,
    source: Uuid,
}

impl Relationship {
    fn new(id: Uuid, variant: Option<RelationshipVariant>, target: Uuid, source: Uuid) -> Self {
        match variant {
            Some(variant) => Self {
                id,
                variant,
                target,
                source,
            },
            None => Self {
                id,
                variant: RelationshipVariant::Related,
                target,
                source,
            },
        }
    }
}

#[derive(PartialEq)]
enum RelationshipVariant {
    Parental = 1,
    Sequential = 2,
    Related = 3,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_minimal_relationship() {
        let relationship = Relationship::new(Uuid::nil(), None, Uuid::nil(), Uuid::nil());

        assert!(relationship.id.is_nil());
        assert!(relationship.target.is_nil());
        assert!(relationship.source.is_nil());
        assert!(relationship.variant as usize == 3)
    }

    #[test]
    fn create_parental() {
        let relationship = Relationship::new(
            Uuid::nil(),
            Some(RelationshipVariant::Parental),
            Uuid::nil(),
            Uuid::nil(),
        );

        assert!(relationship.variant as usize == 1)
    }

    #[test]
    fn create_sequential() {
        let relationship = Relationship::new(
            Uuid::nil(),
            Some(RelationshipVariant::Sequential),
            Uuid::nil(),
            Uuid::nil(),
        );

        assert!(relationship.variant as usize == 2)
    }

    #[test]
    fn create_related() {
        let relationship = Relationship::new(
            Uuid::nil(),
            Some(RelationshipVariant::Related),
            Uuid::nil(),
            Uuid::nil(),
        );

        assert!(relationship.variant as usize == 3)
    }

    #[test]
    fn create_parental_type() {
        let relationship_type = RelationshipVariant::Parental;

        assert!(relationship_type as usize == 1)
    }

    #[test]
    fn create_sequential_type() {
        let sequential = RelationshipVariant::Sequential;

        assert!(sequential as usize == 2)
    }

    #[test]
    fn related() {
        let related_type = RelationshipVariant::Related;

        assert!(related_type as usize == 3)
    }
}
