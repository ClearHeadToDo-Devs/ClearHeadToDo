use std::fmt;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum EdgeDirectionality {
    Directed,
    Undirected,
}

impl fmt::Display for EdgeDirectionality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EdgeDirectionality::Directed => write!(f, "Directed"),
            EdgeDirectionality::Undirected => write!(f, "Undirected"),
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn edge_direction_formatting() {
        let example_edge = EdgeDirectionality::Directed;

        assert!(format!("{}", example_edge) == "Directed")
    }

    #[test]
    fn display_undirected() {
        let example_edge = EdgeDirectionality::Undirected;

        let edge_string = format!("{}", example_edge);

        assert!(edge_string == "Undirected")
    }
}
