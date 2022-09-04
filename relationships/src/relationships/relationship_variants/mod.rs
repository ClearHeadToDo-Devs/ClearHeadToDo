use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::str::FromStr;

pub mod edge_direction;
pub use edge_direction::*;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
#[non_exhaustive]
pub enum RelationshipVariant {
    Parental(EdgeDirectionality),
    Sequential(EdgeDirectionality),
    Related(EdgeDirectionality),
}

impl RelationshipVariant {

    pub fn get_edge_direction(&self) -> String {
        return match self {
            RelationshipVariant::Related(direction) => direction.to_string(),
            RelationshipVariant::Parental(direction) => direction.to_string(),
            RelationshipVariant::Sequential(direction) => direction.to_string(),
        };
    }

    pub fn create_related() -> RelationshipVariant {
        return RelationshipVariant::Related(EdgeDirectionality::Undirected);
    }

    pub fn create_sequential() -> RelationshipVariant {
        return RelationshipVariant::Sequential(EdgeDirectionality::Directed);
    }

    pub fn create_parental() -> RelationshipVariant {
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

impl FromStr for RelationshipVariant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
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
    use serde_test::assert_tokens;
    use serde_test::Token;

    #[test]
    fn related_variant_creation() {
        let related_variant = RelationshipVariant::create_related();

        assert_eq!(related_variant,
            RelationshipVariant::
                Related(EdgeDirectionality::Undirected))
    }

    #[test]
    fn parent_child_variant_creation() {
        let parent_child_variant = RelationshipVariant::create_parental();

        assert_eq!(parent_child_variant,
            RelationshipVariant::
                Parental(EdgeDirectionality::Directed))
    }

    #[test]
    fn previous_subsequest_variant_creation() {
        let previous_subsiquent_variant = RelationshipVariant::create_sequential();

        assert!(
            previous_subsiquent_variant
                == RelationshipVariant::
                    Sequential(EdgeDirectionality::Directed)
        )
    }

    #[test]
    fn create_parent_variant_from_string() {
        let test_variant = RelationshipVariant::from_str("parental").unwrap();

        assert!(test_variant == RelationshipVariant::create_parental())
    }

    #[test]
    fn create_sequential_variant_from_string() {
        let test_variant = RelationshipVariant::from_str("sequential").unwrap();

        assert!(test_variant == RelationshipVariant::Sequential(EdgeDirectionality::Directed))
    }

    #[test]
    fn create_related_variant_from_string() {
        let test_variant = RelationshipVariant::from_str("related").unwrap();

        assert!(test_variant == RelationshipVariant::create_related())
    }

    #[test]
    fn failed_string_variant_creation() {
        let relationship_error =
            RelationshipVariant::from_str("bad variant").unwrap_err();

        assert!(relationship_error.to_string() == "invalid relationship variant")
    }


    #[test]
    fn print_related_string() {
        let relationship_variant = RelationshipVariant::create_related();

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

    #[test]
    fn get_edge_direction_for_directed() {
        let relationship_variant = RelationshipVariant::create_parental();

        let edge_string = relationship_variant.get_edge_direction();

        assert!(edge_string == "Directed")
    }

    #[test]
    fn serialization_and_deserialization() {
        let example_edge = RelationshipVariant::create_related();

        assert_tokens(
            &example_edge,
            &[
                Token::NewtypeVariant {
                    name: "RelationshipVariant",
                    variant: "Related",
                },
                Token::UnitVariant {
                    name: "EdgeDirectionality",
                    variant: "Undirected",
                },
            ],
        )
    }
}
