use std::fmt::Display;

use std::error::Error;
use uuid::Uuid;

#[derive(Debug)]
pub struct Vertex {
    pub id: Uuid,
    pub t: String,
    pub properties: Vec<Property>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            id: Uuid::new_v4(),
            t: "Default".to_string(),
            properties: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: Value,
}

impl Property {
    pub fn new(name: String, value: Value) -> Self {
        Property { name, value }
    }
}

#[derive(PartialEq, Debug)]
pub enum Value {
    Bool(bool),
    Integer(usize),
    String(String),
}

impl TryFrom<Value> for bool {
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(b) => Ok(b),
            _ => Err(ConversionError::WrongInputType),
        }
    }
    type Error = ConversionError;
}

impl TryFrom<Value> for usize {
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => Ok(i),
            _ => Err(ConversionError::WrongInputType),
        }
    }

    type Error = ConversionError;
}

impl TryFrom<Value> for String {
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s),
            _ => Err(ConversionError::WrongInputType),
        }
    }
    type Error = ConversionError;
}

#[derive(Debug)]
pub enum ConversionError {
    WrongInputType,
    InvalidValue,
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConversionError::WrongInputType => write!(f, "Wrong input type"),
            ConversionError::InvalidValue => write!(f, "Value was unable to be Translated"),
        }
    }
}

impl Error for ConversionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default_generic_vertex() {
        let generic_vertex = Vertex::default();

        assert!(generic_vertex.id != Uuid::nil());
        assert!(generic_vertex.t == "Default");
        assert!(generic_vertex.properties.is_empty());
    }

    #[test]
    fn create_bool_value() {
        let generic_value = Value::Bool(true);

        assert!(generic_value == Value::Bool(true));
    }

    #[test]
    fn create_string_value() {
        let generic_value = Value::String("test".to_string());

        assert!(generic_value == Value::String("test".to_string()))
    }

    #[test]
    fn create_int_value() {
        let generic_value = Value::Integer(1);

        assert!(generic_value == Value::Integer(1))
    }

    #[test]
    fn successfully_export_int_value() {
        let generic_value = Value::Integer(1);

        let exported_value: usize = generic_value.try_into().unwrap();

        assert!(exported_value == 1)
    }

    #[test]
    fn failed_string_export_to_int() {
        let generic_value = Value::Bool(true);

        let export_error = usize::try_from(generic_value).unwrap_err();

        assert!(export_error.to_string() == "Wrong input type")
    }

    #[test]
    fn successfully_export_bool_value() {
        let generic_value = Value::Bool(true);

        let exported_value: bool = generic_value.try_into().unwrap();

        assert!(exported_value)
    }

    #[test]
    fn failed_value_export_to_bool() {
        let generic_value = Value::Integer(1);

        let export_error = bool::try_from(generic_value).unwrap_err();

        assert!(export_error.to_string() == "Wrong input type")
    }

    #[test]
    fn successfully_export_string_value() {
        let generic_value = Value::String("test".to_string());

        let exported_value: String = generic_value.try_into().unwrap();

        assert!(exported_value == *"test")
    }

    #[test]
    fn create_empty_named_property() {
        let empty_property = Property::new("test".to_string(), Value::Bool(true));

        assert!(empty_property.name == *"test");
        assert!(empty_property.value == Value::Bool(true))
    }
}
