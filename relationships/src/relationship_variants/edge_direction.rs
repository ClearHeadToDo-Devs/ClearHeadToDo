#[allow(dead_code)]
#[derive(PartialEq)]
pub enum EdgeDirection {
    Directed,
    Undirected,
}

pub trait EdgeDirectionManagement {
    type D: EdgeDirectionManagement;

    fn create_undirected_edge() -> Self::D;
    fn create_directed_edge() -> Self::D;
    fn change_edge_direction(self) -> Self::D;
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
