use crate::graph_storage::edges::GeneralEdge;
use crate::graph_storage::indradb::query_parsing::parse_single_output_query;
use crate::graph_storage::indradb::struct_translation::*;
use crate::graph_storage::querying::GraphEdgeQuerying;
use indradb::Identifier;
use indradb::QueryExt;
use indradb::{Database, MemoryDatastore, Query, SpecificVertexQuery};
use std::error::Error;
use uuid::Uuid;

use crate::graph_storage::indradb::query_parsing::parse_edges;

impl GraphEdgeQuerying for Database<MemoryDatastore> {
    fn get_all_outbound_edges(
        &self,
        id: Uuid,
    ) -> Result<Vec<GeneralEdge>, Box<(dyn std::error::Error + 'static)>> {
        let vertex_query: Query = SpecificVertexQuery::single(id).outbound()?.into();

        let query_output = parse_single_output_query(self, vertex_query)?;

        let concrete_edge_list = parse_edges(query_output)?;

        Ok(convert_edge_list_into_general_edges(concrete_edge_list))
    }

    fn get_all_incoming_edges(&self, id: Uuid) -> Result<Vec<GeneralEdge>, Box<dyn Error>> {
        let vertex_query: Query = SpecificVertexQuery::single(id).inbound()?.into();

        let query_output = parse_single_output_query(self, vertex_query)?;

        let concrete_edge_list = parse_edges(query_output)?;

        Ok(convert_edge_list_into_general_edges(concrete_edge_list))
    }

    fn get_incoming_edges_of_type(
        &self,
        id: Uuid,
        edge_type: &str,
    ) -> Result<Vec<GeneralEdge>, Box<dyn Error>> {
        let vertex_query: Query = SpecificVertexQuery::single(id)
            .inbound()?
            .t(Identifier::new(edge_type).unwrap())
            .into();

        let query_output = parse_single_output_query(self, vertex_query)?;

        let concrete_edge_list = parse_edges(query_output)?;

        Ok(convert_edge_list_into_general_edges(concrete_edge_list))
    }

    fn get_outgoing_edges_of_type(
        &self,
        id: Uuid,
        edge_type: &str,
    ) -> Result<Vec<GeneralEdge>, Box<dyn Error>> {
        let vertex_query: Query = SpecificVertexQuery::single(id)
            .inbound()?
            .t(Identifier::new(edge_type).unwrap())
            .into();

        let query_output = parse_single_output_query(self, vertex_query)?;

        let concrete_edge_list = parse_edges(query_output)?;

        Ok(convert_edge_list_into_general_edges(concrete_edge_list))
    }
}
#[cfg(test)]
mod tests {
    use crate::graph_storage::querying::GraphEdgeQuerying;

    use indradb::{Edge, Identifier};

    use crate::graph_storage::indradb::test_utils::*;
    #[test]
    fn get_all_outgoing_relations() {
        let (database, outbout_vertex, inbound_vertex) = create_database_with_connected_vertices();

        let outgoing_vertex = database.get_all_outbound_edges(outbout_vertex).unwrap();

        assert!(outgoing_vertex[0].get_inbound_id() == inbound_vertex)
    }

    #[test]
    fn get_all_incoming_relations() {
        let (database, outbound_vertex, inbound_vertex) = create_database_with_connected_vertices();

        let incoming_vertex = database.get_all_incoming_edges(inbound_vertex).unwrap();

        assert!(incoming_vertex[0].get_outbound_id() == outbound_vertex)
    }

    #[test]
    fn get_only_incoming_edges_of_type() {
        let (database, outbound_vertex, inbound_vertex) = create_database_with_connected_vertices();

        let new_vertex = database
            .create_vertex_from_type(Identifier::new("test").unwrap())
            .unwrap();
        database
            .create_edge(&Edge::new(
                new_vertex,
                Identifier::new("bad_type").unwrap(),
                outbound_vertex,
            ))
            .unwrap();

        let incoming_vertex = database
            .get_incoming_edges_of_type(inbound_vertex, "test")
            .unwrap();

        assert!(incoming_vertex[0].get_outbound_id() == outbound_vertex && incoming_vertex.len() == 1)
    }

    #[test]
    fn get_only_outgoing_edges_of_type() {
        let (database, outgoing_vertex, incoming_vertex) =
            create_database_with_connected_vertices();
        let new_vertex = database
            .create_vertex_from_type(Identifier::new("test").unwrap())
            .unwrap();
        database
            .create_edge(&Edge::new(
                incoming_vertex,
                Identifier::new("bad_type").unwrap(),
                new_vertex,
            ))
            .unwrap();

        let outgoing_vertices = database
            .get_outgoing_edges_of_type(incoming_vertex, "test")
            .unwrap();

        assert!(outgoing_vertices[0].get_outbound_id() == outgoing_vertex && outgoing_vertices.len() == 1)
    }
}
