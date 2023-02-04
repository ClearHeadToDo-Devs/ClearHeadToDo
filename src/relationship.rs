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

impl From<Relationship> for indradb::EdgeKey {
    fn from(relationship: Relationship) -> Self {
        Self::new(
            relationship.source,
            relationship.variant.into(),
            relationship.target,
        )
    }
}

impl From<indradb::EdgeKey> for Relationship {
    fn from(edge: indradb::EdgeKey) -> Self {
        Self {
            id: Uuid::nil(),
            variant: RelationshipVariant::from(edge.t),
            target: edge.inbound_id,
            source: edge.outbound_id,
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

impl Into<indradb::Identifier> for RelationshipVariant {
    fn into(self) -> indradb::Identifier {
        match self {
            Self::Parental => indradb::Identifier::new("Parental").unwrap(),
            Self::Sequential => indradb::Identifier::new("Sequential").unwrap(),
            Self::Related => indradb::Identifier::new("Related").unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::DateTime;
    use indradb::{EdgeKey, Edge};

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
    fn create_edgekey_from_relationship() {
        let test_relationship = Relationship::new(Uuid::nil(), None, Uuid::nil(), Uuid::nil());

        let converted_edge = indradb::EdgeKey::from(test_relationship);

        assert!(converted_edge.outbound_id.is_nil());
        assert!(converted_edge.inbound_id.is_nil());
        assert!(converted_edge.t == indradb::Identifier::new("Related").unwrap())
    }

    #[test]
    fn create_edge_from_relationship() {
        let test_key: EdgeKey =
            Relationship::new(Uuid::nil(), None, Uuid::nil(), Uuid::nil()).into();

        let cloned_key = test_key.clone();

        let converted_edge = Edge::new(test_key, DateTime::default());

        assert!(converted_edge.key == cloned_key);
    }

    #[test]
    fn create_relationship_from_edge() {
        let test_edge = indradb::EdgeKey::new(
            Uuid::nil(),
            indradb::Identifier::new("Related").unwrap(),
            Uuid::nil(),
        );

        let converted_relationship = Relationship::from(test_edge);

        assert!(converted_relationship.id.is_nil());
        assert!(converted_relationship.target.is_nil());
        assert!(converted_relationship.source.is_nil());
        assert!(converted_relationship.variant as usize == 3)
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

    #[test]
    fn create_identifer_from_sequential_variant() {
        let sequential = RelationshipVariant::Sequential;

        let converted_identifier: indradb::Identifier = RelationshipVariant::into(sequential);

        assert!(converted_identifier.as_str() == "Sequential")
    }

    #[test]
    fn create_identifier_from_parental_variant() {
        let parental = RelationshipVariant::Parental;

        let converted_identifier: indradb::Identifier = RelationshipVariant::into(parental);

        assert!(converted_identifier.as_str() == "Parental")
    }

    #[test]
    fn create_identifier_from_default_variant() {
        let default = RelationshipVariant::default();

        let converted_identifier: indradb::Identifier = RelationshipVariant::into(default);

        assert!(converted_identifier.as_str() == "Related")
    }
}
