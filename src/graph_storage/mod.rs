use crate::action_builder::ActionBuilder;
use crate::priority::Priority;
use core::str::FromStr;
use indradb::{
    Datastore, Identifier, MemoryDatastore, NamedProperty, SpecificVertexQuery, Vertex,
    VertexProperties, VertexProperty, VertexPropertyQuery, VertexQuery,
};
use serde_json::{Number, Value};
use std::error::Error;
use uuid::Uuid;

use crate::action::Action;
use crate::action_interface::{ActionViewing, ActionEditing};

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

pub fn get_action_by_id(datastore: MemoryDatastore, action_id: Uuid) -> Action {
    let extracted_action =
        datastore.get_vertex_properties(create_property_query_for_vertex(action_id, "Name")).unwrap();

ActionBuilder::default()
        .set_name(extracted_action[0].value.as_str().unwrap())
        .build()
}

pub fn add_action_to_datastore(
    action: Action,
    datastore: MemoryDatastore,
) -> Result<(MemoryDatastore, Uuid), Box<dyn Error>> {
    let action_vertex: VertexProperties = action.into();

    datastore.create_vertex(&action_vertex.vertex)?;
    datastore.set_vertex_properties(
        create_property_query_for_vertex(action_vertex.vertex.id, "Name"),
        action_vertex.props[0].value.clone(),
    )?;
    datastore.set_vertex_properties(
        create_property_query_for_vertex(action_vertex.vertex.id, "Completed"),
        action_vertex.props[1].value.clone(),
    )?;
    datastore.set_vertex_properties(
        create_property_query_for_vertex(action_vertex.vertex.id, "Priority"),
        action_vertex.props[2].value.clone(),
    )?;

    Ok((datastore, action_vertex.vertex.id.clone()))
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

pub fn create_property_query_for_vertex(
    action_id: Uuid,
    property_name: &str,
) -> VertexPropertyQuery {
    let vertex_query = create_single_action_query(action_id);

    VertexPropertyQuery::new(vertex_query, create_identifier(property_name))
}

pub fn create_single_action_query(action_id: Uuid) -> VertexQuery {
    SpecificVertexQuery::single(action_id).into()
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

impl From<u64> for Priority{
    fn from(value: u64) -> Self {
match value {
            1 => Priority::Critical,
            2 => Priority::High,
            3 => Priority::Medium,
            4 => Priority::Low,
            5 => Priority::Optional,
            _ => Priority::Optional,
        }
    }
}

#[cfg(test)]
mod test {

    use indradb::{MemoryDatastore, SpecificVertexQuery, VertexPropertyQuery};

    use crate::{action::Action, priority::Priority};

    use super::*;

    fn create_datastore_with_default_action() -> (MemoryDatastore, Uuid) {
        let datastore = MemoryDatastore::default();

        let action = Action::default();

        let (updated_datastore,action_id) = add_action_to_datastore(action, datastore).unwrap();

        (updated_datastore, action_id)
    }

    fn create_datastore_and_action_vertex() -> (MemoryDatastore, VertexProperties) {
        let datatore = MemoryDatastore::default();

        let propertied_vertex: VertexProperties = Action::default().into();

        datatore.create_vertex(&propertied_vertex.vertex).unwrap();

        (datatore, propertied_vertex)
    }

    mod db_ops {
        use super::*;

        #[test]
        fn add_default_action_to_datastore() {
            let test_datastore = MemoryDatastore::default();

            let action = Action::default();

            let (addition_result, actiion_id) = add_action_to_datastore(action, test_datastore).unwrap();

            assert!(addition_result.get_vertex_count().unwrap() == 1)
        }

        #[test]
        fn create_action_from_vertex() {
            let (datastore, action_id) = create_datastore_with_default_action();

            let extracted_action: Action = get_action_by_id(datastore, action_id);

            assert!(extracted_action.get_name() == "Default Action")


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
        fn add_priority_property() {
            let (test_datastore, propertied_vertex) = create_datastore_and_action_vertex();

            let update_result = test_datastore
                .set_vertex_properties(
                    create_property_query_for_vertex(propertied_vertex.vertex.id, "Priority"),
                    Value::Number(Priority::Critical.into()),
                )
                .unwrap();

            assert!(update_result == ())
        }

        #[test]
        fn add_completed_property() {
            let (test_datastore, propertied_vertex) = create_datastore_and_action_vertex();

            let update_result = test_datastore
                .set_vertex_properties(
                    create_property_query_for_vertex(propertied_vertex.vertex.id, "Completed"),
                    Value::Bool(true),
                )
                .unwrap();
        }

        #[test]
        fn add_name_property() {
            let (test_datastore, propertied_vertex) = create_datastore_and_action_vertex();

            let update_result = test_datastore
                .set_vertex_properties(
                    create_property_query_for_vertex(propertied_vertex.vertex.id, "Name"),
                    propertied_vertex.props[0].value.clone(),
                )
                .unwrap();

            assert!(update_result == ())
        }

        #[test]
        fn create_action_vertex_in_datastore() {
            let test_datastore = MemoryDatastore::default();

            let action_vertex: VertexProperties = Action::default().into();

            let vertex_creation_result =
                test_datastore.create_vertex(&action_vertex.vertex).unwrap();

            assert!(vertex_creation_result == true)
        }
    }

    mod db_structs {
        use super::*;

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

    mod db_queries {
        use super::*;

        #[test]
        fn create_example_property_query() {
            let property_query = create_property_query_for_vertex(Uuid::nil(), "test_property");

            assert!(property_query.name.as_str() == "test_property");
            assert!(property_query.inner == create_single_action_query(Uuid::nil()))
        }

        #[test]
        fn create_example_vertex_query() {
            let test_query = create_single_action_query(Uuid::nil());

            assert!(test_query == SpecificVertexQuery::single(Uuid::nil()).into())
        }
    }
}
