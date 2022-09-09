use std::fmt::{Display, Formatter};
use std::fmt;
use std::str::FromStr;
use std::error::Error;

use serde::Deserialize;
use serde::Serialize;

use tabled::Tabled;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, Tabled)]
pub enum EdgeDirectionality {
    Directed,
    Undirected,
}

impl Display for EdgeDirectionality {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

impl FromStr for EdgeDirectionality {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "1" | "directed" | "d" => Ok(EdgeDirectionality::Directed),
            "2" | "undirected" | "u" => Ok(EdgeDirectionality::Undirected),
            "" => Ok(EdgeDirectionality::Undirected),
            _ => Err(Box::new(EdgeDirectionalityError::InvalidInput(s.to_owned()))),
        }
    }
}

#[derive(Debug)]
pub enum EdgeDirectionalityError {
    InvalidInput (String),
}

impl Display for EdgeDirectionalityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<>) -> std::fmt::Result {
        match self {
            EdgeDirectionalityError::InvalidInput(bad_string) => write!(
                f, "{} is an Invalid Priority Option", bad_string),
        }
    }
}

impl std::error::Error for EdgeDirectionalityError {}

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

        let directionality_string = format!("{}", example_edge);

        assert_eq!(directionality_string, "Undirected")
    }



    #[test]
    fn display_directed() {
        let example_edge = EdgeDirectionality::Directed;

        let edge_string = format!("{}", example_edge);

        assert!(edge_string == "Directed")
    }

    #[test]
    fn parse_directed() {
        let directionality_string = "Directed";

        let parsed_directionality = EdgeDirectionality::from_str(&directionality_string).unwrap();

        assert!(parsed_directionality == EdgeDirectionality::Directed)
    }

    #[test]
    fn serialization_and_deserialization() {
        let example_edge = EdgeDirectionality::Undirected;

        let serde_tokens = vec![
            Token::UnitVariant {
                name: "EdgeDirectionality",
                variant: "Undirected",
            },
        ];

        assert_tokens(
            &example_edge, &serde_tokens)
    }
}
