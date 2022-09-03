use serde::Deserialize;
use serde::Serialize;
use std::fmt::{Display, Result, Formatter};

use tabled::Tabled;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, Tabled)]
pub enum EdgeDirectionality {
    Directed,
    Undirected,
}

impl Display for EdgeDirectionality {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            EdgeDirectionality::Directed => write!(f, "Directed"),
            EdgeDirectionality::Undirected => write!(f, "Undirected"),
        }
    }
}

impl Default for EdgeDirectionality {
    fn default() -> Self {
        EdgeDirectionality::Undirected
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use serde_test::assert_tokens;
    use serde_test::Token;

    #[test]
    fn default_directionality(){
        let example_edge = EdgeDirectionality::default();

        assert!(example_edge == EdgeDirectionality::Undirected)
    }

    #[test]
    fn edge_direction_formatting() {
        let example_edge = EdgeDirectionality::default();

        assert_eq!(format!("{}", example_edge) , "Undirected")
    }



    #[test]
    fn display_undirected() {
        let example_edge = EdgeDirectionality::Undirected;

        let edge_string = format!("{}", example_edge);

        assert!(edge_string == "Undirected")
    }

    #[test]
    fn serialization_and_deserialization() {
        let example_edge = EdgeDirectionality::Undirected;

        assert_tokens(
            &example_edge,
            &[Token::UnitVariant {
                name: "EdgeDirectionality",
                variant: "Undirected",
            }],
        )
    }
}
