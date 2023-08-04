use crate::graph_storage::edges::GeneralEdge;
use crate::graph_storage::vertexes::Value;
use crate::graph_storage::vertexes::Vertex;

use std::error::Error;
use uuid::Uuid;
pub trait ConnectedVertexQuerying: MultipleVertexQuerying + GraphEdgeQuerying {
    fn get_all_outbound_vertices(&self, id: Uuid) -> Result<Vec<Vertex>, Box<dyn Error>>;
    fn get_all_inbound_vertices(&self, id: Uuid) -> Result<Vec<Vertex>, Box<dyn Error>>;
}
pub trait SingleVertexQuerying {
    fn get_vertex_type(&self, id: Uuid) -> Result<String, Box<dyn Error>>;
    fn get_property_value(&self, id: Uuid, property_name: String) -> Result<Value, Box<dyn Error>>;
    fn get_all_properties(&self, id: Uuid) -> Result<Vertex, Box<dyn Error>>;
}
pub trait MultipleVertexQuerying {
    fn get_all_vertices(&self) -> Vec<Vertex>;
    fn get_all_vertex_properties(&self) -> Vec<Vertex>;
    fn get_set_of_vertices(&self, vertices: Vec<Uuid>) -> Result<Vec<Vertex>, Box<dyn Error>>;
    fn get_vertices_of_type(&self, filter_type: &str) -> Result<Vec<Vertex>, Box<dyn Error>>;
}

pub trait GraphEdgeQuerying {
    fn get_all_outbound_edges(&self, id: Uuid) -> Result<Vec<GeneralEdge>, Box<dyn Error>>;
    fn get_all_incoming_edges(&self, id: Uuid) -> Result<Vec<GeneralEdge>, Box<dyn Error>>;
    fn get_incoming_edges_of_type(
        &self,
        id: Uuid,
        edge_type: &str,
    ) -> Result<Vec<GeneralEdge>, Box<dyn Error>>;
    fn get_outgoing_edges_of_type(
        &self,
        id: Uuid,
        edge_type: &str,
    ) -> Result<Vec<GeneralEdge>, Box<dyn Error>>;
}
