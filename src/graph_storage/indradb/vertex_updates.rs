use crate::graph_storage::{updates::VertexUpdates, vertexes::Value};
use indradb::{Database, Identifier, MemoryDatastore, SpecificVertexQuery};
use std::error::Error;
use uuid::Uuid;

impl VertexUpdates for Database<MemoryDatastore> {
    fn set_vertex_property(
        &self,
        vertex_id: Uuid,
        property_name: &str,
        property_value: Value,
    ) -> Result<(), Box<dyn Error>> {
        self.set_properties(
            SpecificVertexQuery::single(vertex_id),
            Identifier::new(property_name)?,
            &property_value.try_into()?,
        )?;

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::{graph_storage::indradb::test_utils::*, graph_storage::vertexes::Value};
    use crate::graph_storage::updates::VertexUpdates;

    use indradb::{
        AllVertexQuery, CountQueryExt, Json, MemoryDatastore, QueryExt, QueryOutputValue,
        SpecificVertexQuery,
    };

    use uuid::Uuid;
    #[test]
    fn successfully_update_property() {
        let (database, vertex_id) = create_database_with_propertied_vertex();

        database
            .set_vertex_property(vertex_id, "test_property", Value::Bool(false))
            .unwrap();

        let property_query = &database
            .get(SpecificVertexQuery::single(vertex_id).properties().unwrap())
            .unwrap()[0];

        let output = match property_query {
            QueryOutputValue::VertexProperties(properties) => properties,
            _ => panic!("Expected vertex properties"),
        };

        assert!(output[0].props[0].value == Json::new(false.into()));
    }

    #[test]
    fn update_property_without_vertex() {
        let database = MemoryDatastore::new_db();

        database
            .set_vertex_property(Uuid::nil(), "test_property", Value::Bool(false))
            .unwrap();

        assert!(
            database.get(AllVertexQuery.count().unwrap()).unwrap()[0] == QueryOutputValue::Count(0)
        )
    }

    #[test]
    fn update_property_that_doesnt_exist() {
        let (database, vertex) = create_database_with_test_vertex();

        database
            .set_vertex_property(vertex, "bad_property", Value::Bool(false))
            .unwrap();

        let test_output = &database
            .get(SpecificVertexQuery::single(vertex).properties().unwrap())
            .unwrap()[0];

        let output = match test_output {
            QueryOutputValue::VertexProperties(properties) => properties,
            _ => panic!("Expected vertex properties"),
        };

        assert!(output.len() == 1)
    }
}
