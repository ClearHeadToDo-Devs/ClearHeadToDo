use uuid::Uuid;

use indradb;

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

impl Default for RelationshipVariant {
    fn default() -> Self {
        Self::Related
    }
}

impl From<indradb::Identifier> for RelationshipVariant {
    fn from(identifier: indradb::Identifier) -> Self {
        match identifier.as_str() {
            "Parental" => Self::Parental,
            "Sequential" => Self::Sequential,
            "Related" => Self::Related,
            _ => Self::Related,
        }
    }
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
    fn create_parental_variant() {
        let relationship_type = RelationshipVariant::Parental;

        assert!(relationship_type as usize == 1)
    }

    #[test]
    fn create_sequential_variant() {
        let sequential = RelationshipVariant::Sequential;

        assert!(sequential as usize == 2)
    }

    #[test]
    fn create_related_variant() {
        let related_type = RelationshipVariant::Related;

        assert!(related_type as usize == 3)
    }

    #[test]
    fn create_default_variant() {
        let default_type = RelationshipVariant::default();

        assert!(default_type as usize == 3)
    }

    #[test]
    fn create_parental_variant_from_identifier() {
        let test_identifier = indradb::Identifier::new("Parental").unwrap();

        let converted_relationship_variant = RelationshipVariant::from(test_identifier);

        assert!(converted_relationship_variant as usize == 1)
    }

    #[test]
    fn create_sequential_variant_from_identifier() {
        let test_identifier = indradb::Identifier::new("Sequential").unwrap();

        let converted_relationship_variant = RelationshipVariant::from(test_identifier);

        assert!(converted_relationship_variant as usize == 2)
    }

    #[test]
    fn create_related_variant_from_identifier() {
        let test_identifier = indradb::Identifier::new("Related").unwrap();

        let converted_relationship_variant = RelationshipVariant::from(test_identifier);

        assert!(converted_relationship_variant as usize == 3)
    }

    #[test]
    fn create_related_from_other_variant() {
        let test_identifier = indradb::Identifier::new("Other").unwrap();

        let converted_relationship_variant = RelationshipVariant::from(test_identifier);

        assert!(converted_relationship_variant as usize == 3)
    }
}
