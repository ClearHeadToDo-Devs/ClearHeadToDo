use crate::action::Action;
use indradb::{VertexProperty, VertexProperties, Vertex, Identifier};

impl From<Action> for VertexProperties{
    fn from(value: Action) -> VertexProperties {
        let vertex = Vertex::new(Identifier::new("Action".to_string()).unwrap());
        let properties = vec![];
    }
}

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn create_propertied_vertex_from_action() {
        let test_action = Action::default();

        let converted_propertied_vertex: VertexProperties = VertexProperties::from(test_action);
    }
}
