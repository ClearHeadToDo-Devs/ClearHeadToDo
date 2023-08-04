use crate::graph_storage::db_error::GraphDatabaseError;
use crate::graph_storage::indradb::{
    query_building::{create_single_propertied_vertex_query, create_single_vertex_query},
    query_parsing::*,
};
use crate::graph_storage::querying::*;
use crate::graph_storage::vertexes::*;

use indradb::{
    AllVertexQuery, Database, Identifier, MemoryDatastore, NamedProperty, Query, QueryExt,
    RangeVertexQuery, SpecificVertexQuery, Vertex as IndradbVertex
};

use std::error::Error;
use uuid::Uuid;

impl ConnectedVertexQuerying for Database<MemoryDatastore> {
    fn get_all_outbound_vertices(&self, id: Uuid) -> Result<Vec<Vertex>, Box<dyn Error>> {
        let outbound_edge_list = &self.get_all_outbound_edges(id)?;

        let mut outbound_vertices_id_list = vec![];
        for edge in outbound_edge_list {
            outbound_vertices_id_list.push(edge.get_inbound_id());
        }

        self.get_set_of_vertices(outbound_vertices_id_list)
    }
    fn get_all_inbound_vertices(&self, id: Uuid) -> Result<Vec<Vertex>, Box<dyn Error>> {
        let edge_list = self.get_all_incoming_edges(id)?;

        let mut inbound_vertices_id_list = vec![];
        for edge in edge_list {
            inbound_vertices_id_list.push(edge.get_outbound_id());
        }

        self.get_set_of_vertices(inbound_vertices_id_list)
    }
}
impl SingleVertexQuerying for Database<MemoryDatastore> {
    fn get_vertex_type(&self, id: Uuid) -> Result<String, Box<dyn Error>> {
        let query: Query = create_single_vertex_query(id).into();

        let query_output = parse_single_output_query(self, query)?;

        Ok(parse_vertices(query_output)?[0].t.to_string())
    }
    fn get_all_properties(&self, id: Uuid) -> Result<Vertex, Box<dyn Error>> {
        let query: Query = create_single_propertied_vertex_query(id).into();

        let query_value = parse_single_output_query(self, query)?;
        let indradb_vertex = parse_propertied_vertices(query_value)?[0].clone();
        Ok(indradb_vertex.try_into()?)
    }
    fn get_property_value(&self, id: Uuid, property_name: String) -> Result<Value, Box<dyn Error>> {
        let query: Query = create_single_propertied_vertex_query(id)
            .name(Identifier::new(property_name)?)
            .into();

        let query_value = parse_single_output_query(self, query)?;

        let propertied_vertex = parse_propertied_vertices(query_value)?[0].clone();

        let indradb_property: NamedProperty = propertied_vertex
            .props
            .first()
            .ok_or(GraphDatabaseError::InvalidProperty)?
            .clone();
        let generic_value: Property = indradb_property.try_into()?;

        Ok(generic_value.value)
    }
}
impl MultipleVertexQuerying for Database<MemoryDatastore> {
    fn get_all_vertices(&self) -> Vec<Vertex> {
        let query_output_list = self.get(Query::AllVertex).unwrap();

        let query_output = query_output_list.first().unwrap();

        let indradb_vertices = parse_vertices(query_output.clone()).unwrap_or(vec![]);

        convert_indradb_vertices_to_generic(indradb_vertices)
    }
    fn get_all_vertex_properties(&self) -> Vec<Vertex> {
        let all_properties_query: Query =
            AllVertexQuery::properties(AllVertexQuery).unwrap().into();

        let query_output = parse_single_output_query(self, all_properties_query).unwrap();

        let indradb_vertices = parse_propertied_vertices(query_output).unwrap_or(vec![]);

        indradb_vertices.iter().map(|v| Vertex::try_from(v.clone()).unwrap()).collect()
    
    }

    fn get_set_of_vertices(&self, vertices: Vec<Uuid>) -> Result<Vec<Vertex>, Box<dyn Error>> {
        let query = SpecificVertexQuery::new(vertices);

        let query_output = parse_single_output_query(self, query.into())?;

        let db_vertices = parse_vertices(query_output)?;

        Ok(convert_indradb_vertices_to_generic(db_vertices))
    }

    fn get_vertices_of_type(&self, filter_type: &str) -> Result<Vec<Vertex>, Box<dyn Error>> {
        let query: Query = RangeVertexQuery::new()
            .t(Identifier::new(filter_type)?)
            .into();

        let query_output = parse_single_output_query(self, query)?;

        let indradb_vertices = parse_vertices(query_output)?;

        Ok(convert_indradb_vertices_to_generic(indradb_vertices))
    }
}

pub fn convert_indradb_vertices_to_generic(vertices: Vec<IndradbVertex>) -> Vec<Vertex> {
    let mut generic_vertices = vec![];
    for vertex in vertices {
        generic_vertices.push(vertex.into())
    }
    generic_vertices
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_storage::indradb::test_utils::*;
    use indradb::{Json, MemoryDatastore};
    use serde_json::Value;
    use uuid::Uuid;

    mod vertex_type {
        use super::*;
        #[test]
        fn successful_read_vertex_type() {
            let (database, new_vertex_id) = create_database_with_test_vertex();

            let queried_vertex_type = database.get_vertex_type(new_vertex_id).unwrap();

            assert!(queried_vertex_type == "test")
        }
        #[test]
        fn failed_read_vertex_type() {
            let database = MemoryDatastore::new_db();

            let vertex_query_error = database.get_vertex_type(Uuid::new_v4()).unwrap_err();

            assert!(vertex_query_error.to_string() == "Empty Output")
        }
    }
    mod vertex_properties {
        use super::*;
        mod single {
            use crate::graph_storage::vertexes::Value;
            use indradb::SpecificVertexQuery;

            use super::*;
            #[test]
            fn failed_read_vertex_bad_property() {
                let (database, vertex_id) = create_database_with_test_vertex();

                let vertex_query_error = database
                    .get_property_value(vertex_id, "bad_property".to_string())
                    .unwrap_err();

                assert!(vertex_query_error.to_string() == "Empty Output");
            }
            #[test]
            fn successful_read_vertex_property() {
                let (database, new_vertex_id) = create_database_with_propertied_vertex();

                let queried_vector_property = database
                    .get_property_value(new_vertex_id, "test_property".to_string())
                    .unwrap();

                assert!(queried_vector_property == Value::Bool(true))
            }

            #[test]
            fn successful_read_even_if_not_first() {
                let (database, new_vertex_id) = create_database_with_propertied_vertex();
                database
                    .set_properties(
                        SpecificVertexQuery::single(new_vertex_id),
                        create_identifier("second_test_prop"),
                        &Json::new(false.into()),
                    )
                    .unwrap();

                let queried_vector_property = database
                    .get_property_value(new_vertex_id, "second_test_prop".to_string())
                    .unwrap();

                assert!(queried_vector_property == Value::Bool(false))
            }
        }
        mod multiple {
            use super::*;
            #[test]
            fn successful_get_all_vertex_properties() {
                let (database, vertex_id) = create_database_with_multi_propertied_vertex();

                let queried_vector_properties = database.get_all_properties(vertex_id).unwrap();

                assert!(queried_vector_properties.properties.len() == 2)
            }
            #[test]
            fn failed_get_vertex_properties() {
                let (database, vertex_id) = create_database_with_test_vertex();

                let vertex_query_error = database.get_all_properties(vertex_id).unwrap_err();

                assert!(vertex_query_error.to_string() == "Empty Output");
            }
        }
    }
    mod multiple_vertices {
        use super::*;
        #[test]
        fn get_all_vertices_empty() {
            let database = MemoryDatastore::new_db();

            let query_results = database.get_all_vertices();

            assert!(query_results.is_empty());
        }
        #[test]
        fn get_multiple_vertices() {
            let (database, _) = create_database_with_multiple_vertices();

            let vertex_list = database.get_all_vertices();

            assert!(vertex_list.len() == 2)
        }
        #[test]
        fn get_all_vertex_properties() {
            let (database, _vertex_id, _vertex_id_2) =
                create_database_with_multiple_propertied_vertices();

            let queried_vector_properties = database.get_all_vertex_properties();

            assert!(
                queried_vector_properties.len() == 2
                    && queried_vector_properties[0].properties[0].value == Value::Bool(true).try_into().unwrap()
                    && queried_vector_properties[1].properties[0].value == Value::Bool(true).try_into().unwrap()
            )
        }
        #[test]
        fn get_small_set_of_vertices() {
            let (database, vertices) = create_database_with_multiple_vertices();
            database
                .create_vertex_from_type(create_identifier("test"))
                .unwrap();

            let vertex_list = database.get_set_of_vertices(vertices).unwrap();

            assert!(vertex_list.len() == 2)
        }
        #[test]
        fn failed_read_vertex_set() {
            let database = MemoryDatastore::new_db();

            let vertex_query_error = database.get_set_of_vertices(vec![Uuid::nil()]).unwrap_err();

            assert!(vertex_query_error.to_string() == "Empty Output");
        }
        #[test]
        fn read_vertex_of_type() {
            let (database, _) = create_database_with_test_vertex();
            database
                .create_vertex_from_type(create_identifier("irrelevant"))
                .unwrap();

            let vertex_list = database.get_vertices_of_type("test").unwrap();

            assert!(vertex_list.len() == 1)
        }
    }
    mod connected_vertices {
        use super::*;

        #[test]
        fn get_all_outbound_vertices() {
            let (database, outbound_id, inbound_id) = create_database_with_connected_vertices();

            let outbound_vertices = database.get_all_outbound_vertices(outbound_id).unwrap();

            assert!(outbound_vertices[0].id == inbound_id)
        }

        #[test]
        fn get_all_inbound_vertices() {
            let (database, outbound_id, inbound_id) = create_database_with_connected_vertices();

            let inbound = database.get_all_inbound_vertices(inbound_id).unwrap();

            assert!(inbound[0].id == outbound_id)
        }
    }
}
