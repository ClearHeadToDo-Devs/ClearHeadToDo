use super::vertexes::*;

use crate::action::interface::*;
use crate::action::builder::*;
use crate::action::Action;

impl From<Vertex> for Action {
    fn from(value: Vertex) -> Self {
        let mut builder = ActionBuilder::default();

        builder.set_id(value.id).build()
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn convert_vertex_into_action() {
        let vertex = Vertex::default();

        let test_action: Action = vertex.clone().into();

        assert!(test_action.get_id() == vertex.id)
    }
}
