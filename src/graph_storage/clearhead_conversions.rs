use std::str::FromStr;

use super::vertexes::*;

use crate::action::priority::Priority;
use crate::action::interface::*;
use crate::action::builder::*;
use crate::action::Action;

impl From<Vertex> for Action {
    fn from(value: Vertex) -> Self {
        let mut builder = ActionBuilder::default();

        let name: String = value.properties[0].value.clone().try_into().unwrap();
        let completion_status: bool = value.properties[1].value.clone().try_into().unwrap();
        let priority: Priority = value.properties[2].value.clone().try_into().unwrap();

        builder.set_id(value.id).set_name(&name).set_completion_status(completion_status).set_priority(priority).build()
    }
}

impl TryFrom<Value> for Priority {
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(_) => Err(ConversionError::WrongInputType),
            Value::Integer(int) => Ok(Priority::from_repr(int).unwrap()),
            Value::String(string) => Ok(Priority::from_str(&string).unwrap())
        }
    }
    type Error = ConversionError;
}


#[cfg(test)]
mod tests{
    use uuid::Uuid;
    use super::*;
    
    #[test]
    fn convert_vertex_into_action() {
        let vertex = Vertex {
            id: Uuid::new_v4(),
            properties: vec![
                Property::new("name".to_string(), Value::String("test".to_string())),
                Property::new("completed".to_string(), Value::Bool(true)),
                Property::new("priority".to_string(), Value::Integer(1))],
            t: "Action".to_string(),
        };

        let test_action: Action = vertex.clone().into();

        assert!(test_action.get_id() == vertex.id
        && test_action.get_name() == "test"
        && test_action.get_completion_status()
        && *test_action.get_priority() == Priority::Critical)
    }

    #[test]
    fn convert_string_value_into_priority(){
        let priorty_value = Value::String("Optional".to_string());

        let test_priority: Priority = priorty_value.try_into().unwrap();

        assert!(test_priority == Priority::Optional);
    }

    #[test]
    fn convert_usize_into_priority(){
        let priority_value = Value::Integer(1);

        let test_priority: Priority = priority_value.try_into().unwrap();

        assert!(test_priority == Priority::Critical);
    }
}
