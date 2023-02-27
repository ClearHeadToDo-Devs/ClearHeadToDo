use crate::Action;

use std::io::Write;
use std::path::PathBuf;
use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;
use xdg::*;

use std::io::Error;

fn get_clearhead_database_path(file_name: &str) -> PathBuf {
    let clearhead_directories = get_clearhead_base_directories().unwrap();

    return clearhead_directories.get_data_home().join(file_name);
}
fn get_clearhead_base_directories() -> Result<BaseDirectories, BaseDirectoriesError> {
    let clearhead_directories = BaseDirectories::with_prefix("clearhead")?;

    Ok(clearhead_directories)
}

#[cfg(test)]
mod test {
    use indradb::{Datastore, MemoryDatastore};

    use super::*;

    #[test]
    fn read_empty_data_store() {
        let path = get_clearhead_database_path("test.db");

        let datastore: MemoryDatastore = MemoryDatastore::read_msgpack(path).unwrap();

        assert!(datastore.sync().is_ok())
    }

    #[test]
    fn create_empty_datastore() {
        let path = get_clearhead_database_path("test.db");

        let datastore: MemoryDatastore = MemoryDatastore::create_msgpack(path).unwrap();

        assert!(datastore.sync().is_ok())
    }

    #[test]
    fn get_datastore_location() {
        let path = get_clearhead_database_path("test.db");

        assert!(path.to_str().unwrap().contains("test.db"));
    }

    #[test]
    fn return_clearhead_directory() {
        let directory = get_clearhead_base_directories().unwrap();

        assert!(directory
            .get_data_home()
            .to_str()
            .unwrap()
            .contains("clearhead"));
    }
}
