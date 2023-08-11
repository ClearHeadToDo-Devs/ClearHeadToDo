use std::fmt::Display;

use std::error::Error;
use uuid::Uuid;

#[derive(Debug)]
pub struct Vertex {
    pub id: Uuid,
    pub t: String,
    pub properties: Vec<Property>,
}

impl Vertex {
    fn new(id: Uuid, t: &str, properties: &[Property]) -> Self {
        Vertex {
            id,
            t: t.to_string(),
            properties: properties.to_vec(),
        }
    }
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

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: Value,
}

impl Property {
    pub fn new(name: String, value: Value) -> Self {
        Property { name, value }
    }
}

#[derive(PartialEq, Debug, Clone)]
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
    fn create_custom_vertex() {
        let custom_vertex = Vertex::new(Uuid::nil(), "test", &[Property::new("test property".to_string(), Value::Bool(true))]);

        let exported_value: bool = custom_vertex.properties[0].value.clone().try_into().unwrap();

        assert!(custom_vertex.id == Uuid::nil()
        && custom_vertex.t == "test"
        && custom_vertex.properties[0].name == "test property"
        && exported_value)
    }

    #[test]
    fn create_string_value() {
        let generic_value = Value::String("test".to_string());

        let exported_value: String = generic_value.try_into().unwrap();

        assert!(exported_value == *"test")
    }

    #[test]
    fn create_int_value() {
        let generic_value = Value::Integer(1);

        let exported_value: usize = generic_value.try_into().unwrap();

        assert!(exported_value == 1)
    }

    #[test]
    fn failed_export() {
        let generic_value = Value::Bool(true);

        let export_error = usize::try_from(generic_value).unwrap_err();

        assert!(export_error.to_string() == "Wrong input type")
    }
}
