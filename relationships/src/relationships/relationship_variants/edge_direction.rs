use std::fmt;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum EdgeDirection {
    Directed,
    Undirected,
}

impl fmt::Display for EdgeDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EdgeDirection::Directed => write!(f, "Directed"),
            EdgeDirection::Undirected => write!(f, "Undirected"),
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn edge_direction_formatting() {
        let example_edge = EdgeDirection::Directed;

        assert!(format!("{}", example_edge) == "Directed")
    }

    #[test]
    fn display_undirected() {
        let example_edge = EdgeDirection::Undirected;

        let edge_string = format!("{}", example_edge);

        assert!(edge_string == "Undirected")
    }
}
