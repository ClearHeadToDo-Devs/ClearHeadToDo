use core::str::FromStr;
use indradb::{Identifier, Vertex, VertexProperty};
use serde_json::{Value, Number};
use uuid::Uuid;
use crate::priority::Priority;
use indradb::VertexProperties;


pub fn create_string_property(vertex_id: Uuid, value: Value) -> VertexProperty {
    VertexProperty::new(vertex_id, value)
}

pub fn create_boolean_property(vertex_id: Uuid, value: bool) -> VertexProperty {
    VertexProperty::new(vertex_id, Value::Bool(value))
}

pub fn create_numeric_property(vertex_id: Uuid, value: Number)->VertexProperty{
    VertexProperty::new(vertex_id, Value::Number(value))
}

pub fn create_action_vertex() -> Vertex {
    Vertex::new(create_identifier("Action"))
}

pub fn create_string_json_value(str: &str) -> Value {
    Value::String(str.to_string())
}

pub fn create_identifier(str: &str) -> Identifier {
    Identifier::from_str(str).unwrap()
}

impl From<Priority> for Number {
    fn from(priority: Priority) -> Self {
        match priority {
            Priority::Critical => Number::from(1),
            Priority::High => Number::from(2),
            Priority::Medium => Number::from(3),
            Priority::Low => Number::from(4),
            Priority::Optional => Number::from(5)
        }
    }
}


#[cfg(test)]
mod test {


    use crate::priority::Priority;

    use super::*;

    #[test]
    fn create_action_property() {
        let test_vertex = create_action_vertex();

        let name_property = create_string_property(test_vertex.id, create_string_json_value("test name"));
        let completed_property = create_boolean_property(test_vertex.id, false);
        let priority_property = create_numeric_property(test_vertex.id, Priority::Critical.into());

        let property_vector = vec![name_property, completed_property, priority_property];



    }

    #[test]
    fn create_name_property() {
        let test_vertex = create_action_vertex();

        let name_property =
            create_string_property(test_vertex.id, create_string_json_value("test name"));

        assert!(name_property.value == create_string_json_value("test name"))
    }

    #[test]
    fn create_completed_property() {
        let test_vertex = create_action_vertex();

        let completed_property = create_boolean_property(test_vertex.id, false);

        assert!(completed_property.value.as_bool().unwrap() == false)
    }

    #[test]
    fn create_priority_property() {
        let test_vertex = create_action_vertex();

let priority_property =
        create_numeric_property(test_vertex.id, Priority::Critical.into());

        assert!(priority_property.value.as_u64().unwrap() == 1)

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

    #[test]
    fn create_example_identifier() {
        let example_identifier = create_identifier("example");

        assert!(example_identifier.as_str() == "example")
    }
}
