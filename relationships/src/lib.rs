use uuid::Uuid;

#[allow(dead_code)]
pub struct Relationship {
    id: Uuid,
    variant: RelationshipVariant,
    participant_1: Uuid,
    participant_2: Uuid,
}

#[allow(dead_code)]
#[derive(PartialEq)]
#[non_exhaustive]
enum RelationshipVariant {
    ParentChild(EdgeDirection),
    PreviousSubsiquent(EdgeDirection),
    Related(EdgeDirection),
}

#[allow(dead_code)]
#[derive(PartialEq)]
enum EdgeDirection {
    Directed,
    Undirected,
}

trait RelationshipManagement {
    #[allow(dead_code)]
    fn create_new(variant: RelationshipVariant, participant_1: Uuid, participant_2: Uuid) -> Self;
}

trait RelationshipVariantManagement {
    type V: RelationshipVariantManagement;

    fn create_related_variant() -> Self::V;
    fn create_previous_subsequent_variant() -> Self::V;
    fn create_parent_child_variant() -> Self::V;
    fn change_variant_edge_direction(self) -> Self::V;
}

trait EdgeDirectionManagement {
    type D: EdgeDirectionManagement;
    fn create_undirected_edge() -> Self::D;
    fn create_directed_edge() -> Self::D;
    fn change_edge_direction(self) -> Self::D;
}

impl RelationshipManagement for Relationship {
    fn create_new(variant: RelationshipVariant, participant_1: Uuid, participant_2: Uuid) -> Self {
        let relationship_id = Uuid::new_v4();
        return Relationship {
            id: relationship_id,
            variant,
            participant_1,
            participant_2,
        };
    }
}

#[allow(dead_code)]
impl RelationshipVariantManagement for RelationshipVariant {
    type V = RelationshipVariant;
    fn create_related_variant() -> RelationshipVariant {
        return RelationshipVariant::Related(EdgeDirection::create_undirected_edge());
    }

    fn create_previous_subsequent_variant() -> RelationshipVariant {
        return RelationshipVariant::PreviousSubsiquent(EdgeDirection::create_directed_edge());
    }

    fn create_parent_child_variant() -> RelationshipVariant {
        return RelationshipVariant::ParentChild(EdgeDirection::create_directed_edge());
    }

    fn change_variant_edge_direction(self) -> RelationshipVariant {
        match self {
            RelationshipVariant::Related(direction) => {
                RelationshipVariant::Related(direction.change_edge_direction())
            }
            RelationshipVariant::PreviousSubsiquent(direction) => {
                RelationshipVariant::PreviousSubsiquent(direction.change_edge_direction())
            }
            RelationshipVariant::ParentChild(direction) => {
                RelationshipVariant::ParentChild(direction.change_edge_direction())
            }
        }
    }
}

#[allow(dead_code)]
impl EdgeDirectionManagement for EdgeDirection {
    type D = EdgeDirection;
    fn create_directed_edge() -> EdgeDirection {
        return EdgeDirection::Directed;
    }

    fn create_undirected_edge() -> EdgeDirection {
        return EdgeDirection::Undirected;
    }

    fn change_edge_direction(self) -> EdgeDirection {
        match self {
            EdgeDirection::Undirected => EdgeDirection::Directed,
            EdgeDirection::Directed => EdgeDirection::Undirected,
        }
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
        let nil_id = Uuid::nil();

        let nil_relationship = create_nil_relationship(
            RelationshipVariant::create_related_variant(),
            nil_id,
            nil_id,
        );

        assert!(nil_relationship.id == Uuid::nil());
    }

    #[test]
    fn undirected_relationship_creation() {
        let nil_participant_id = Uuid::nil();

        let nil_relationship = Relationship::create_new(
            RelationshipVariant::create_related_variant(),
            nil_participant_id,
            nil_participant_id,
        );

        assert!(
            nil_relationship.variant == RelationshipVariant::Related(EdgeDirection::Undirected)
        );
    }

    #[test]
    fn directed_relationship_creation() {
        let nil_participant_id = Uuid::nil();

        let nil_relationship = Relationship::create_new(
            RelationshipVariant::create_related_variant(),
            nil_participant_id,
            nil_participant_id,
        );

        assert!(
            nil_relationship.variant == RelationshipVariant::Related(EdgeDirection::Undirected)
        );
    }

    #[test]
    fn unique_relationship_participants() {
        let first_participant_id = Uuid::new_v4();
        let second_participant_id = Uuid::new_v4();
        let direction = RelationshipVariant::create_related_variant();

        let relationship =
            Relationship::create_new(direction, first_participant_id, second_participant_id);

        assert!(relationship.participant_2 != relationship.participant_1)
    }

    #[test]
    fn unique_relationship_id() {
        let nil_participant_id = Uuid::nil();

        let relationship_1 = Relationship::create_new(
            RelationshipVariant::create_related_variant(),
            nil_participant_id,
            nil_participant_id,
        );
        let relationship_2 = Relationship::create_new(
            RelationshipVariant::create_related_variant(),
            nil_participant_id,
            nil_participant_id,
        );

        assert!(relationship_2.id != relationship_1.id);
    }

    #[test]
    fn related_variant_creation() {
        let related_variant = RelationshipVariant::create_related_variant();

        assert!(related_variant == RelationshipVariant::Related(EdgeDirection::Undirected))
    }

    #[test]
    fn parent_child_variant_creation() {
        let parent_child_variant = RelationshipVariant::create_parent_child_variant();

        assert!(parent_child_variant == RelationshipVariant::ParentChild(EdgeDirection::Directed))
    }

    #[test]
    fn previous_subsequest_variant_creation() {
        let previous_subsiquent_variant = RelationshipVariant::create_previous_subsequent_variant();

        assert!(
            previous_subsiquent_variant
                == RelationshipVariant::PreviousSubsiquent(EdgeDirection::Directed)
        )
    }

    #[test]
    fn change_variant() {
        let example_variant = RelationshipVariant::Related(EdgeDirection::Undirected);
    }

    #[test]
    fn change_variant_edge_direction() {
        let example_variant = RelationshipVariant::Related(EdgeDirection::Undirected);
        let altered_variant = example_variant.change_variant_edge_direction();

        assert!(altered_variant == RelationshipVariant::Related(EdgeDirection::Directed))
    }

    #[test]
    fn directed_edge_creation() {
        let directed_edge = EdgeDirection::create_directed_edge();

        assert!(directed_edge == EdgeDirection::Directed)
    }

    #[test]
    fn undirected_edge_creation() {
        let undirected_edge = EdgeDirection::create_undirected_edge();

        assert!(undirected_edge == EdgeDirection::Undirected)
    }

    #[test]
    fn change_edge_direction() {
        let example_edge = EdgeDirection::Undirected;
        let altered_edge = example_edge.change_edge_direction();

        assert!(altered_edge == EdgeDirection::Directed)
    }
}
