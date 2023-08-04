use indradb::NamedProperty;
use std::error::Error;
use uuid::Uuid;

pub trait VertexCreation {
    fn create_vertex_by_type(&self, vertex_type: String) -> Result<Uuid, Box<dyn Error>>;
    fn create_vertex_by_type_with_properties(
        &self,
        vertex_type: String,
        properties: Vec<NamedProperty>,
    ) -> Result<Uuid, Box<dyn Error>>;
}
