use uuid::Uuid;

use strum_macros::*;

#[derive(Debug, Clone)]

pub struct Relationship {
    id: Uuid,
    variant: RelationshipVariant,
    target: Uuid,
    source: Uuid,
}

impl Relationship {
    pub fn new(id: Uuid, variant: Option<RelationshipVariant>, target: Uuid, source: Uuid) -> Self {
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

#[derive(PartialEq, Clone, Debug, EnumString)]
pub enum RelationshipVariant {
    Parental = 1,
    Sequential = 2,
    Related = 3,
}

impl Default for RelationshipVariant {
    fn default() -> Self {
        Self::Related
    }
}


#[cfg(test)]
mod test {

    use super::*;
    use rstest::*;

    #[fixture]
    fn empty_relationship() -> Relationship {
        Relationship::new(Uuid::nil(), None, Uuid::nil(), Uuid::nil())
}
    fn create_empty_relationship(variant:  RelationshipVariant)-> Relationship {
        Relationship::new(Uuid::nil(), Option::Some(variant), Uuid::nil(), Uuid::nil())
    }

    #[rstest]
    fn create_minimal_relationship(empty_relationship: Relationship) {
        assert!(empty_relationship.id.is_nil());
        assert!(empty_relationship.target.is_nil());
        assert!(empty_relationship.source.is_nil());
        assert!(empty_relationship.variant as usize == 3)
    }


    #[test]
    fn create_parental(){
        let relationship = create_empty_relationship(RelationshipVariant::Parental);

        assert!(relationship.variant as usize == 1)
    }

    #[test]
    fn create_sequential() {
        let relationship = create_empty_relationship(RelationshipVariant::Sequential);
        assert!(relationship.variant as usize == 2)
    }
}

