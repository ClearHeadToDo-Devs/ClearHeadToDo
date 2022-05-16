pub mod relationship_variants;
pub use relationship_variants::*;

use uuid::Uuid;

use im::Vector;

trait RelationshipListManagement {
    type L: RelationshipListManagement;
    fn add_related(&mut self, participant_1: Uuid, participant_2: Uuid);
    fn add_sequential(&mut self, participant_1: Uuid, participant_2: Uuid);
    fn add_parental(&mut self, participant_1: Uuid, participant_2: Uuid);
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub struct Relationship {
    id: Uuid,
    variant: RelationshipVariant,
    participant_1: Uuid,
    participant_2: Uuid,
}

trait RelationshipManagement {
    type R: RelationshipManagement;
    type V: RelationshipVariantManagement;
    #[allow(dead_code)]
    fn create_new(
        variant_str: &str,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Result<Self::R, String>;

    fn create_new_related(participant_1: Uuid, participant_2: Uuid) -> Self::R;
    fn create_new_sequential(participant_1: Uuid, participant_2: Uuid) -> Self::R;
    fn create_new_parental(participant_1: Uuid, participant_2: Uuid) -> Self::R;
}

impl RelationshipManagement for Relationship {
    type R = Relationship;
    type V = RelationshipVariant;

    fn create_new(
        variant_str: &str,
        participant_1: Uuid,
        participant_2: Uuid,
    ) -> Result<Self, String> {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::create_from_string(variant_str)?;
        return Ok(Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        });
    }
    fn create_new_related(participant_1: Uuid, participant_2: Uuid) -> Self {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::create_related();
        Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        }
    }

    fn create_new_sequential(participant_1: Uuid, participant_2: Uuid) -> Self {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::create_sequential();
        Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        }
    }

    fn create_new_parental(participant_1: Uuid, participant_2: Uuid) -> Self {
        let id = Uuid::new_v4();
        let variant = RelationshipVariant::create_parental();
        Relationship {
            id,
            variant,
            participant_1,
            participant_2,
        }
    }
}

impl RelationshipListManagement for Vector<Relationship> {
    type L = Vector<Relationship>;

    fn add_related(&mut self, participant_1: Uuid, participant_2: Uuid) {
        let new_relationship = Relationship::create_new_related(participant_1, participant_2);
        self.push_back(new_relationship)
    }

    fn add_sequential(&mut self, participant_1: Uuid, participant_2: Uuid) {
        let new_relationship = Relationship::create_new_sequential(participant_1, participant_2);
        self.push_back(new_relationship)
    }

    fn add_parental(&mut self, participant_1: Uuid, participant_2: Uuid) {
        let new_relationship = Relationship::create_new_parental(participant_1, participant_2);
        self.push_back(new_relationship)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_nil_relationship(
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

    #[test]
    fn relationship_id_creation() {
        let nil_relationship = create_nil_relationship(
            RelationshipVariant::Related(EdgeDirection::Undirected),
            Uuid::nil(),
            Uuid::nil(),
        );

        assert!(nil_relationship.id == Uuid::nil());
    }

    #[test]
    fn unique_relationship_id() {
        let relationship_1 = Relationship::create_new("related", Uuid::nil(), Uuid::nil()).unwrap();
        let relationship_2 = Relationship::create_new("related", Uuid::nil(), Uuid::nil()).unwrap();

        assert!(relationship_2.id != relationship_1.id);
    }

    #[test]
    fn invalid_relationship_variant_input() {
        let invalid_relationship =
            Relationship::create_new("bad variant", Uuid::nil(), Uuid::nil()).unwrap_err();

        assert!(invalid_relationship == "invalid relationship variant");
    }

    #[test]
    fn create_related_relationship() {
        let new_related_relationship = Relationship::create_new_related(Uuid::nil(), Uuid::nil());

        assert!(
            new_related_relationship.variant
                == RelationshipVariant::Related(EdgeDirection::Undirected)
        )
    }

    #[test]
    fn create_subsequent() {
        let new_sequential_relationship =
            Relationship::create_new_sequential(Uuid::nil(), Uuid::nil());

        assert!(
            new_sequential_relationship.variant
                == RelationshipVariant::Sequential(EdgeDirection::Directed)
        );
    }

    #[test]
    fn create_parental() {
        let new_parental_relationship = Relationship::create_new_parental(Uuid::nil(), Uuid::nil());

        assert!(
            new_parental_relationship.variant
                == RelationshipVariant::Parental(EdgeDirection::Directed)
        )
    }

    #[test]
    fn add_related_relationship_to_list() {
        let mut relationship_list: Vector<Relationship> = Vector::new();

        relationship_list.add_related(Uuid::nil(), Uuid::nil());

        assert! {relationship_list[0].variant == RelationshipVariant::Related(EdgeDirection::Undirected)}
    }

    #[test]
    fn add_sequential_relationship_to_list() {
        let mut relationship_list: Vector<Relationship> = Vector::new();

        relationship_list.add_sequential(Uuid::nil(), Uuid::nil());

        assert! {relationship_list[0].variant == RelationshipVariant::Sequential(EdgeDirection::Directed)}
    }

    #[test]
    fn add_parental_relationship_to_list() {
        let mut relationship_list: Vector<Relationship> = Vector::new();

        relationship_list.add_parental(Uuid::nil(), Uuid::nil());

        assert!(
            relationship_list[0].variant == RelationshipVariant::Parental(EdgeDirection::Directed)
        )
    }

    #[test]
    fn remove_relationship() {
        let mut relationship_list: Vector<Relationship> = Vector::new();
        relationship_list.add_related(Uuid::nil(), Uuid::nil());

        relationship_list.pop_back();

        assert!(relationship_list.len() == 0);
    }

    #[test]
    fn remove_first_relationship() {
        let mut relationship_list: Vector<Relationship> = Vector::new();
        relationship_list.add_related(Uuid::nil(), Uuid::nil());
        relationship_list.add_related(Uuid::nil(), Uuid::nil());

        relationship_list.pop_front();

        assert!(
            relationship_list[0].variant == RelationshipVariant::Related(EdgeDirection::Undirected)
        );
    }

    #[test]
    fn remove_middle_relationship() {
        let mut relationship_list: Vector<Relationship> = Vector::new();
        relationship_list.add_sequential(Uuid::nil(), Uuid::nil());
        relationship_list.add_related(Uuid::nil(), Uuid::nil());
        relationship_list.add_parental(Uuid::nil(), Uuid::nil());

        relationship_list.remove(1);

        assert!(
            relationship_list[0].variant
                == RelationshipVariant::Sequential(EdgeDirection::Directed)
                && relationship_list.len() == 2
        );
    }
}
