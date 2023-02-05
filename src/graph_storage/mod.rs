use indradb::{Identifier, Vertex, VertexProperty};
use serde_json::Value;
use uuid::Uuid;

pub fn create_string_property(vertex_id: Uuid, value: Value) -> VertexProperty {
    VertexProperty::new(vertex_id, value)
}

pub fn create_action_vertex() -> Vertex {
    Vertex::new(Identifier::new("Action").unwrap())
}

pub fn create_string_json_value(str: &str) -> Value {
    Value::String(str.to_string())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn create_name_property() {
        let test_vertex = create_action_vertex();

        let name_property =
            create_string_property(test_vertex.id, create_string_json_value("test name"));
    }

    #[test]
    fn create_bare_action_vertex() {
        let action_vertex = create_action_vertex();

        assert!(action_vertex.t == Identifier::new("Action").unwrap());
    }

    #[test]
    fn create_example_string_value() {
        let test_value = create_string_json_value("example");

        assert!(test_value == Value::String("example".to_string()))
    }
}
