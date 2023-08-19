use indradb::{Database, MemoryDatastore, Datastore};

struct LocalClearHeadService {
    graph_db: Database<MemoryDatastore>
}

impl LocalClearHeadService {
    fn new(database: Database<MemoryDatastore>) -> Self{
        LocalClearHeadService { graph_db: database }
    }
}


#[cfg(test)]
mod test{
    use crate::graph_storage::indradb::local_creation_and_reading::create_local_indradb_database;

    use super::*;


    #[test]
    fn create_clearhead_service(){
        let service = LocalClearHeadService::new(create_local_indradb_database(None));

        assert!(service.graph_db.sync().is_ok())
    }

}
