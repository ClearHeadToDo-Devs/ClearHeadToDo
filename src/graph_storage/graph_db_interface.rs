use core::str::FromStr;
use indradb::RangeVertexQuery;
use indradb::{EdgeProperties,VertexProperties, Identifier, MemoryDatastore, Vertex};
use serde_json::{Number, Value};
use std::error::Error;
use uuid::Uuid;

pub trait GraphDatabaseQuerying {
    fn get_vertex(&self, id: Uuid) -> Result<VertexProperties, Box<dyn Error>>;
    fn get_edge(&self, id: Uuid) -> Result<EdgeProperties, Box<dyn Error>>;
}

impl GraphDatabaseQuerying for Database {
    fn get_vertex(&self, id: Uuid) -> Result<VertexProperties, Box<dyn Error>>{
        self.get_vertex(id);

    }
}


#[cfg(test)]
mod test{
use super::*;
    #[test]
    fn getIndraVertex() {
        let datastore = MemoryDatastore::new();

        datastore.create_vertex(Vertex::FromStr("Test"));

        let queried_vertex = datastore
            .get_vertex(Identifier::from_str("Test").unwrap())
            .unwrap();

        assert!(queried_vertex.name=="Test");
    }
}
