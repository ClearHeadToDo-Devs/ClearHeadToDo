use std::fmt;

pub mod edge_direction;
pub use edge_direction::*;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
#[non_exhaustive]
pub enum RelationshipVariant {
    Parental(EdgeDirectionality),
    Sequential(EdgeDirectionality),
    Related(EdgeDirectionality),
}

pub trait RelationshipVariantManagement {
    type V: RelationshipVariantManagement;

    fn create_from_string(target_variant: &str) -> Result<Self::V, String>;

    fn create_related() -> Self::V;
    fn create_sequential() -> Self::V;
    fn create_parental() -> Self::V;

    fn get_edge_direction(&self) -> String;
}

#[allow(dead_code)]
impl RelationshipVariantManagement for RelationshipVariant {
    type V = RelationshipVariant;

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

    fn get_edge_direction(&self) -> String {
        return match self {
            RelationshipVariant::Related(direction) => direction.to_string(),
            RelationshipVariant::Parental(direction) => direction.to_string(),
            RelationshipVariant::Sequential(direction) => direction.to_string(),
        };
    }

    fn create_related() -> RelationshipVariant {
        return RelationshipVariant::Related(EdgeDirectionality::Undirected);
    }

    fn create_sequential() -> RelationshipVariant {
        return RelationshipVariant::Sequential(EdgeDirectionality::Directed);
    }

    fn create_parental() -> RelationshipVariant {
        return RelationshipVariant::Parental(EdgeDirectionality::Directed);
    }
}

impl fmt::Display for RelationshipVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            RelationshipVariant::Related(edge) => write!(f, "Related: {:?}", edge),
            RelationshipVariant::Parental(edge) => write!(f, "Parental: {:?}", edge),
            RelationshipVariant::Sequential(edge) => {
                write!(f, "Sequential: {:?}", edge)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_parent_variant_from_string() {
        let test_variant = RelationshipVariant::create_from_string("parental").unwrap();

        assert!(test_variant == RelationshipVariant::Parental(EdgeDirectionality::Directed))
    }

    #[test]
    fn create_sequential_variant_from_string() {
        let test_variant = RelationshipVariant::create_from_string("sequential").unwrap();

        assert!(test_variant == RelationshipVariant::Sequential(EdgeDirectionality::Directed))
    }

    #[test]
    fn create_related_variant_from_string() {
        let test_variant = RelationshipVariant::create_from_string("related").unwrap();

        assert!(test_variant == RelationshipVariant::Related(EdgeDirectionality::Undirected))
    }

    #[test]
    fn failed_string_variant_creation() {
        let relationship_error =
            RelationshipVariant::create_from_string("bad variant").unwrap_err();

        assert!(relationship_error.to_string() == "invalid relationship variant")
    }

    #[test]
    fn related_variant_creation() {
        let related_variant = RelationshipVariant::create_related();

        assert!(related_variant == RelationshipVariant::Related(EdgeDirectionality::Undirected))
    }

    #[test]
    fn parent_child_variant_creation() {
        let parent_child_variant = RelationshipVariant::create_parental();

        assert!(parent_child_variant == RelationshipVariant::Parental(EdgeDirectionality::Directed))
    }

    #[test]
    fn previous_subsequest_variant_creation() {
        let previous_subsiquent_variant = RelationshipVariant::create_sequential();

        assert!(
            previous_subsiquent_variant
                == RelationshipVariant::Sequential(EdgeDirectionality::Directed)
        )
    }

    #[test]
    fn print_related_string() {
        let relationship_variant = RelationshipVariant::Related(EdgeDirectionality::Undirected);

        assert!(format!("{}", relationship_variant) == "Related: Undirected")
    }

    #[test]
    fn print_parental_string() {
        let relationship_variant = RelationshipVariant::create_parental();

        assert!(format!("{}", relationship_variant) == "Parental: Directed")
    }

    #[test]
    fn print_sequential_string() {
        let relationship_variant = RelationshipVariant::create_sequential();

        assert!(format!("{}", relationship_variant) == "Sequential: Directed")
    }

    #[test]
    fn print_edge_direction() {
        let relationship_variant = RelationshipVariant::create_related();

        let edge_string = relationship_variant.get_edge_direction();

        assert!(edge_string == "Undirected")
    }
}
