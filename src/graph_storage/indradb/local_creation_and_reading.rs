use crate::graph_storage::local_db_create_and_open::*;
use indradb::{Database, MemoryDatastore};
use std::{error::Error, ffi::OsString};

pub fn create_local_indradb_database(location: Option<OsString>) -> Database<MemoryDatastore> {
    <Database<MemoryDatastore> as GraphDatabaseCreation>::create_database(location)
}

pub fn read_local_indradb_database(
    location: OsString,
) -> Result<Database<MemoryDatastore>, Box<dyn Error>> {
    <Database<MemoryDatastore> as GraphDatabaseCreation>::read_database(location)
}

impl GraphDatabaseCreation for Database<MemoryDatastore> {
    fn create_database(location: Option<OsString>) -> Database<MemoryDatastore> {
        match location {
            None => MemoryDatastore::new_db(),
            Some(path) => MemoryDatastore::create_msgpack_db(path),
        }
    }
    fn read_database(location: OsString) -> Result<Database<MemoryDatastore>, Box<dyn Error>> {
        Ok(MemoryDatastore::read_msgpack_db(location)?)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    mod create {
        use super::*;
        #[test]
        fn create_database() {
            let database = create_local_indradb_database(None);

            assert!(database.sync().is_ok());
        }
        #[test]
        fn create_file_database() {
            let database =
                create_local_indradb_database(Some("src/graph_storage/indradb/data/test_write.db".into()));

            assert!(database.sync().is_ok())
        }
    }
    mod read {
        use super::*;
        #[test]
        fn successfully_read_file_database() {
            let db_read_result =
                read_local_indradb_database("src/graph_storage/indradb/data/test_read.db".into());

            let database = db_read_result.unwrap();

            assert!(database.sync().is_ok())
        }
        #[test]
        fn failed_read_file_database() {
            let database_error = read_local_indradb_database("bad path".into()).err();

            assert!(database_error.is_some())
        }
    }
}
