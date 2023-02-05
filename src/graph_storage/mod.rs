use crate::{action::Action, action_interface::ActionViewing, priority::Priority};
use indradb::{Identifier, Vertex, VertexProperties, VertexProperty, Json};
use serde_json::{Value, Number};

impl From<Action> for VertexProperties {
    fn from(value: Action) -> VertexProperties {
        let vertex = Vertex::new(Identifier::new("Action".to_string()).unwrap());
        let vertex_id = vertex.id.clone();

        let properties = vec![
            VertexProperty::new(vertex_id, Value::String(value.get_name().to_string())),
            VertexProperty::new(vertex_id, Value::Bool(value.get_completion_status())),
            VertexProperty::new(vertex_id, Value::Number(value.get_priority().into()))
        ];
    }
}

impl From<&Priority> for Value {
    fn from(val: &Priority) -> Self {
        Value::Number(1)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn create_propertied_vertex_from_action() {
        let test_action = Action::default();

        let converted_propertied_vertex: VertexProperties = VertexProperties::from(test_action);
    }
}
