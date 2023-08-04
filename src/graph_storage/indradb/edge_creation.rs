use crate::graph_storage::edges::EdgeCreation;
use indradb::{Database, Edge, Identifier, MemoryDatastore};
use uuid::Uuid;

impl EdgeCreation for Database<MemoryDatastore> {
    fn append_edge(&self, outbound_id: Uuid, t: &str, inbound_id: Uuid) -> bool {
        let edge = Edge::new(outbound_id, Identifier::new(t).unwrap(), inbound_id);

        self.create_edge(&edge).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::graph_storage::indradb::test_utils::create_database_with_multiple_vertices;
use crate::graph_storage::indradb::test_utils::*;

    use crate::graph_storage::edges::EdgeCreation;
    use indradb::MemoryDatastore;
    use uuid::Uuid;

    #[test]
    fn create_sample_link() {
        let (database, vertex_id) = create_database_with_multiple_vertices();

        let link_created = database.append_edge(vertex_id[0], "test", vertex_id[1]);

        assert!(link_created)
    }

    #[test]
    fn failed_sample_link() {
        let database = MemoryDatastore::new_db();

        let link_created = database.append_edge(Uuid::nil(), "test", Uuid::nil());

        assert!(!link_created);
    }
}
