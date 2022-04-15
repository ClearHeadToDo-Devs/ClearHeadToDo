pub mod edge_direction;
pub use edge_direction::*;

#[allow(dead_code)]
#[derive(PartialEq)]
#[non_exhaustive]
pub enum RelationshipVariant {
    ParentChild(EdgeDirection),
    PreviousSubsequent(EdgeDirection),
    Related(EdgeDirection),
}

pub trait RelationshipVariantManagement {
    type V: RelationshipVariantManagement;

    fn create_related_variant() -> Self::V;
    fn create_previous_subsequent_variant() -> Self::V;
    fn create_parent_child_variant() -> Self::V;
    fn change_variant_edge_direction(self) -> Self::V;
    fn change_variant_type(self, target_variant: Self::V) -> Self::V;
    fn create_variant_from_string(target_variant: &str) -> Self::V;
}

#[allow(dead_code)]
impl RelationshipVariantManagement for RelationshipVariant {
    type V = RelationshipVariant;

    fn create_related_variant() -> RelationshipVariant {
        return RelationshipVariant::Related(EdgeDirection::create_undirected_edge());
    }

    fn create_previous_subsequent_variant() -> RelationshipVariant {
        return RelationshipVariant::PreviousSubsequent(EdgeDirection::create_directed_edge());
    }

    fn create_parent_child_variant() -> RelationshipVariant {
        return RelationshipVariant::ParentChild(EdgeDirection::create_directed_edge());
    }

    fn change_variant_edge_direction(self) -> RelationshipVariant {
        match self {
            RelationshipVariant::Related(direction) => {
                RelationshipVariant::Related(direction.change_edge_direction())
            }
            RelationshipVariant::PreviousSubsequent(direction) => {
                RelationshipVariant::PreviousSubsequent(direction.change_edge_direction())
            }
            RelationshipVariant::ParentChild(direction) => {
                RelationshipVariant::ParentChild(direction.change_edge_direction())
            }
        }
    }

    fn change_variant_type(self, target_variant: RelationshipVariant) -> RelationshipVariant {
        match target_variant {
            RelationshipVariant::Related(direction) => {
                return RelationshipVariant::Related(direction)
            }
            RelationshipVariant::ParentChild(direction) => {
                return RelationshipVariant::ParentChild(direction)
            }
            RelationshipVariant::PreviousSubsequent(direction) => {
                return RelationshipVariant::PreviousSubsequent(direction)
            }
        }
    }

    fn create_variant_from_string(target_variant: &str) -> RelationshipVariant {
        match target_variant {
            "parent" | "child" | "Parent/Child" | "PC" | "P/C" | "parental" => {
                return RelationshipVariant::create_parent_child_variant()
            }
            "previous" | "subsequent" | "Previous/Subsequent" | "PS" | "P/S" | "sequential" => {
                return RelationshipVariant::create_previous_subsequent_variant()
            }
            _ => return RelationshipVariant::create_related_variant(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

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
                == RelationshipVariant::PreviousSubsequent(EdgeDirection::Directed)
        )
    }

    #[test]
    fn change_variant_edge_direction() {
        let example_variant = RelationshipVariant::Related(EdgeDirection::Undirected);
        let altered_variant = example_variant.change_variant_edge_direction();

        assert!(altered_variant == RelationshipVariant::Related(EdgeDirection::Directed))
    }

    #[test]
    fn change_variant_type() {
        let example_variant = RelationshipVariant::Related(EdgeDirection::Undirected);
        let altered_variant = example_variant
            .change_variant_type(RelationshipVariant::ParentChild(EdgeDirection::Undirected));

        assert!(altered_variant == RelationshipVariant::ParentChild(EdgeDirection::Undirected))
    }

    #[test]
    fn create_parent_variant_from_string() {
        let test_variant = RelationshipVariant::create_variant_from_string("parental");

        assert!(test_variant == RelationshipVariant::ParentChild(EdgeDirection::Directed))
    }

    #[test]
    fn create_sequential_variant_from_string() {
        let test_variant = RelationshipVariant::create_variant_from_string("sequential");

        assert!(test_variant == RelationshipVariant::PreviousSubsequent(EdgeDirection::Directed))
    }
}
