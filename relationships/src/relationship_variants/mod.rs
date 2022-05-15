pub mod edge_direction;
pub use edge_direction::*;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
#[non_exhaustive]
pub enum RelationshipVariant {
    Parental(EdgeDirection),
    Sequential(EdgeDirection),
    Related(EdgeDirection),
}

pub trait RelationshipVariantManagement {
    type V: RelationshipVariantManagement;

    fn create_related() -> Self::V;
    fn create_sequential() -> Self::V;
    fn create_parental() -> Self::V;

    fn change_edge_direction(self) -> Self::V;
    fn change_type(self, target_variant: Self::V) -> Self::V;

    fn create_from_string(target_variant: &str) -> Result<Self::V, String>;
}

#[allow(dead_code)]
impl RelationshipVariantManagement for RelationshipVariant {
    type V = RelationshipVariant;

    fn create_related() -> RelationshipVariant {
        return RelationshipVariant::Related(EdgeDirection::create_undirected());
    }

    fn create_sequential() -> RelationshipVariant {
        return RelationshipVariant::Sequential(EdgeDirection::create_directed());
    }

    fn create_parental() -> RelationshipVariant {
        return RelationshipVariant::Parental(EdgeDirection::create_directed());
    }

    fn change_edge_direction(self) -> RelationshipVariant {
        match self {
            RelationshipVariant::Related(direction) => {
                RelationshipVariant::Related(direction.change_direction())
            }
            RelationshipVariant::Sequential(direction) => {
                RelationshipVariant::Sequential(direction.change_direction())
            }
            RelationshipVariant::Parental(direction) => {
                RelationshipVariant::Parental(direction.change_direction())
            }
        }
    }

    fn change_type(self, target_variant: RelationshipVariant) -> RelationshipVariant {
        match target_variant {
            RelationshipVariant::Related(direction) => {
                return RelationshipVariant::Related(direction)
            }
            RelationshipVariant::Parental(direction) => {
                return RelationshipVariant::Parental(direction)
            }
            RelationshipVariant::Sequential(direction) => {
                return RelationshipVariant::Sequential(direction)
            }
        }
    }

    fn create_from_string(target_variant: &str) -> Result<RelationshipVariant, String> {
        match target_variant {
            "parent" | "child" | "Parent/Child" | "PC" | "P/C" | "parental" => {
                return Ok(RelationshipVariant::create_parental())
            }
            "previous" | "subsequent" | "Previous/Subsequent" | "PS" | "P/S" | "sequential" => {
                return Ok(RelationshipVariant::create_sequential())
            }
            "related" | "relational" | "generic" => {
                return Ok(RelationshipVariant::create_related())
            }
            _ => return Err("invalid relationship variant".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn related_variant_creation() {
        let related_variant = RelationshipVariant::create_related();

        assert!(related_variant == RelationshipVariant::Related(EdgeDirection::Undirected))
    }

    #[test]
    fn parent_child_variant_creation() {
        let parent_child_variant = RelationshipVariant::create_parental();

        assert!(parent_child_variant == RelationshipVariant::Parental(EdgeDirection::Directed))
    }

    #[test]
    fn previous_subsequest_variant_creation() {
        let previous_subsiquent_variant = RelationshipVariant::create_sequential();

        assert!(
            previous_subsiquent_variant == RelationshipVariant::Sequential(EdgeDirection::Directed)
        )
    }

    #[test]
    fn change_variant_edge_direction() {
        let example_variant = RelationshipVariant::Related(EdgeDirection::Undirected);
        let altered_variant = example_variant.change_edge_direction();

        assert!(altered_variant == RelationshipVariant::Related(EdgeDirection::Directed))
    }

    #[test]
    fn change_variant_type() {
        let example_variant = RelationshipVariant::Related(EdgeDirection::Undirected);
        let altered_variant =
            example_variant.change_type(RelationshipVariant::Parental(EdgeDirection::Undirected));

        assert!(altered_variant == RelationshipVariant::Parental(EdgeDirection::Undirected))
    }

    #[test]
    fn create_parent_variant_from_string() {
        let test_variant = RelationshipVariant::create_from_string("parental").unwrap();

        assert!(test_variant == RelationshipVariant::Parental(EdgeDirection::Directed))
    }

    #[test]
    fn create_sequential_variant_from_string() {
        let test_variant = RelationshipVariant::create_from_string("sequential").unwrap();

        assert!(test_variant == RelationshipVariant::Sequential(EdgeDirection::Directed))
    }

    #[test]
    fn create_related_variant_from_string() {
        let test_variant = RelationshipVariant::create_from_string("related").unwrap();

        assert!(test_variant == RelationshipVariant::Related(EdgeDirection::Undirected))
    }

    #[test]
    fn failed_string_variant_creation() {
        let relationship_error =
            RelationshipVariant::create_from_string("bad variant").unwrap_err();

        assert!(relationship_error.to_string() == "invalid relationship variant")
    }
}
