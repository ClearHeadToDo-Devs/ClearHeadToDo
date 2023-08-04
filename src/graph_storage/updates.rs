use crate::graph_storage::vertexes::Value;

use std::error::Error;
use uuid::Uuid;
pub trait VertexUpdates {
    fn set_vertex_property(
        &self,
        vertex_id: Uuid,
        property_name: &str,
        property_value: Value,
    ) -> Result<(), Box<dyn Error>>;
}
