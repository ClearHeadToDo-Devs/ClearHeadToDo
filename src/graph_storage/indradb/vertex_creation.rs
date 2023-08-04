use crate::graph_storage::insertion::VertexCreation;
use indradb::{Database, Identifier, MemoryDatastore, NamedProperty, SpecificVertexQuery};
use std::error::Error;
use uuid::Uuid;

impl VertexCreation for Database<MemoryDatastore> {
    fn create_vertex_by_type(&self, vertex_type: String) -> Result<Uuid, Box<dyn Error>> {
        let vertex_id = self.create_vertex_from_type(Identifier::new(vertex_type).unwrap())?;

        Ok(vertex_id)
    }

    fn create_vertex_by_type_with_properties(
        &self,
        vertex_type: String,
        properties: Vec<NamedProperty>,
    ) -> Result<Uuid, Box<dyn Error>> {
        let vertex_id = self.create_vertex_by_type(vertex_type)?;

        for property in properties {
            self.set_properties(
                SpecificVertexQuery::single(vertex_id),
                property.name,
                &property.value,
            )?;
        }

        Ok(vertex_id)
    }
}
#[cfg(test)]
mod tests {
    use crate::graph_storage::insertion::VertexCreation;

    use indradb::{Json, MemoryDatastore, NamedProperty};

    use crate::graph_storage::indradb::test_utils::*;

    #[test]
    fn create_sample_vertex() {
        let database = MemoryDatastore::new_db();

        let new_vertex_id = database.create_vertex_by_type("test".to_string()).unwrap();

        assert!(!new_vertex_id.is_nil())
    }

    #[test]
    fn create_vertex_with_properties() {
        let database = MemoryDatastore::new_db();

        let new_vertex_id = database
            .create_vertex_by_type_with_properties(
                "test".to_string(),
                vec![NamedProperty::new(
                    create_identifier("test_property"),
                    Json::new(true.into()),
                )],
            )
            .unwrap();

        let vertex_properties = get_property_for_vertex(&database, new_vertex_id);

        assert!(
            vertex_properties.props[0].name.to_string() == "test_property"
                && vertex_properties.props[0].value == Json::new(true.into())
        )
    }
}
