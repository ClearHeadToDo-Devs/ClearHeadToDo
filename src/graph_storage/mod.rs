use crate::priority::Priority;
use core::str::FromStr;
use indradb::{Datastore, Identifier, NamedProperty, Vertex, VertexProperties, VertexProperty};
use serde_json::{Number, Value};

use crate::action::Action;
use crate::action_interface::ActionViewing;

impl From<Action> for VertexProperties {
    fn from(value: Action) -> Self {
        let vertex = create_action_vertex();

        let name_property = create_name_property(value.get_name());
        let completed_property = create_completed_property(value.get_completion_status());
        let priority_property = create_priority_property(value.get_priority().to_owned().into());

        VertexProperties::new(
            vertex,
            vec![name_property, completed_property, priority_property],
        )
    }
}

pub fn create_name_property(value: &str) -> NamedProperty {
    let name_identifier = create_identifier("Name");
    let string_value = create_string_json_value(value);

    NamedProperty::new(name_identifier, string_value)
}

pub fn create_completed_property(value: bool) -> NamedProperty {
    NamedProperty::new(create_identifier("completed"), Value::Bool(value))
}

pub fn create_priority_property(value: Number) -> NamedProperty {
    NamedProperty::new(create_identifier("Priority"), Value::Number(value))
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
            Priority::Optional => Number::from(5),
        }
    }
}

#[cfg(test)]
mod test {

    use indradb::{MemoryDatastore, SpecificVertexQuery, VertexPropertyQuery};

    use crate::{action::Action, priority::Priority};

    use super::*;

    fn create_datastore_and_action_vertex() -> (MemoryDatastore, VertexProperties) {
        let datatore = MemoryDatastore::default();

        let propertied_vertex: VertexProperties = Action::default().into();

        datatore.create_vertex(&propertied_vertex.vertex).unwrap();

        (datatore, propertied_vertex)
    }

    #[test]
    fn add_name_property() {
        let (test_datastore, propertied_vertex) = create_datastore_and_action_vertex();

        let update_result = test_datastore
            .set_vertex_properties(
                VertexPropertyQuery::new(
                    SpecificVertexQuery::single(propertied_vertex.vertex.id).into(),
                    propertied_vertex.props[0].name.clone(),
                ),
                propertied_vertex.props[0].value.clone(),
            )
            .unwrap();

        assert!(update_result == ())
    }

    #[test]
    fn create_action_vertex_in_datastore() {
        let test_datastore = MemoryDatastore::default();

        let action_vertex: VertexProperties = Action::default().into();

        let vertex_creation_result = test_datastore.create_vertex(&action_vertex.vertex).unwrap();

        assert!(vertex_creation_result == true)

    }

    #[test]
    fn create_full_action_vertex_example() {
        let action = Action::default();

        let test_propertied_vertex: VertexProperties = action.into();

        assert!(test_propertied_vertex.vertex.t == create_identifier("Action"));
        assert!(test_propertied_vertex.props[0].name.as_str() == "Name");
        assert!(test_propertied_vertex.props[0].value.as_str().unwrap() == "Default Action");
        assert!(test_propertied_vertex.props[1].name.as_str() == "completed");
        assert!(test_propertied_vertex.props[1].value.as_bool().unwrap() == false);
        assert!(test_propertied_vertex.props[2].name.as_str() == "Priority");
        assert!(test_propertied_vertex.props[2].value.as_u64().unwrap() == 5)
    }

    #[test]
    fn create_example_name_property() {
        let name_property = create_name_property("test name");

        assert!(name_property.name.as_str() == "Name");
        assert!(name_property.value.as_str().unwrap() == "test name")
    }

    #[test]
    fn create_example_completed_property() {
        let completed_property = create_completed_property(false);

        assert!(completed_property.name.as_str() == "completed");
        assert!(completed_property.value.as_bool().unwrap() == false)
    }

    #[test]
    fn create_example_priority_property() {
        let priority_property = create_priority_property(Priority::Critical.into());

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
