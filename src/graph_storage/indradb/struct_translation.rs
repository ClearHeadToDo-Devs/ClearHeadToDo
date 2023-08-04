use indradb::Identifier;
use crate::graph_storage::edges::GeneralEdge;
use crate::graph_storage::vertexes::*;
use indradb::Edge;
use indradb::Json;
use indradb::NamedProperty;
use indradb::Vertex as IndradbVertex;
use indradb::VertexProperties;
use serde_json::Value as SerdeValue;

impl TryFrom<VertexProperties> for Vertex {
    fn try_from(value: VertexProperties) -> Result<Self, Self::Error> {
        let mut properties = vec![];
        for property in value.props {
            properties.push(property.try_into()?);
        }
        Ok(Vertex {
            id: value.vertex.id,
            t: value.vertex.t.to_string(),
            properties,
        })
    }
    type Error = ConversionError;
}
impl From<IndradbVertex> for Vertex {
    fn from(value: IndradbVertex) -> Self {
        Vertex {
            id: value.id,
            t: value.t.to_string(),
            properties: vec![],
        }
    }
}
impl TryFrom<NamedProperty> for Property {
    fn try_from(value: NamedProperty) -> Result<Self, Self::Error> {
        let property_value = &value.value.0;

        if property_value.is_boolean() {
            Ok(Property::new(
                value.name.to_string(),
                Value::Bool(value.value.as_bool().unwrap()),
            ))
        } else if property_value.is_string() {
            return Ok(Property::new(
                value.name.to_string(),
                Value::String(value.value.as_str().unwrap().to_string()),
            ));
        } else if property_value.is_number() {
            return Ok(Property::new(
                value.name.to_string(),
                Value::Integer(value.value.as_u64().unwrap().try_into().unwrap()),
            ));
        } else {
            return Err(ConversionError::WrongInputType);
        }
    }
    type Error = ConversionError;
}
impl TryFrom<SerdeValue> for Value {
    fn try_from(value: SerdeValue) -> Result<Self, Self::Error> {
        match value {
            SerdeValue::Number(numb) => {
                Ok(Value::Integer(numb.as_u64().unwrap().try_into().unwrap()))
            }
            SerdeValue::Bool(bool) => Ok(Value::Bool(bool)),
            SerdeValue::String(string) => Ok(Value::String(string)),
            SerdeValue::Null => Err(ConversionError::WrongInputType),
            SerdeValue::Array(_) => Err(ConversionError::WrongInputType),
            SerdeValue::Object(_) => Err(ConversionError::WrongInputType),
        }
    }
    type Error = ConversionError;
}
impl From<Edge> for GeneralEdge {
    fn from(edge: Edge) -> Self {
        GeneralEdge::new(edge.outbound_id, edge.t.to_string(), edge.inbound_id)
    }
}
impl TryFrom<Value> for Json {
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(int) => Ok(Json::new(int.into())),
            Value::String(string) => Ok(Json::new(string.into())),
            Value::Bool(bool) => Ok(Json::new(bool.into())),
        }
    }

    type Error = ConversionError;
}
impl From<Property> for NamedProperty {
    fn from(property: Property) -> Self {
        Self {
            name: Identifier::new(property.name).unwrap(),
            value: match property.value {
                Value::Integer(int) => Json::new(int.into()),
                Value::String(string) => Json::new(string.into()),
                Value::Bool(bool) => Json::new(bool.into()),
            },
        }
    }
}


pub fn convert_edge_list_into_general_edges(edge_list: Vec<Edge>) -> Vec<GeneralEdge> {
    edge_list.into_iter().map(|edge| edge.into()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use indradb::ijson;


    #[test]
    fn convert_generic_value_to_json(){
        let value = Value::Integer(1);

        let json_value: Json = value.try_into().unwrap();

        assert!(json_value == ijson!(1))
    }
}
