use std::path::PathBuf;

use indradb::MemoryDatastore;
use xdg::*;

pub fn get_clearhead_datastore(datastore_name: &str) -> MemoryDatastore {
    let path = get_clearhead_database_path(datastore_name);

    match MemoryDatastore::read_msgpack(path.clone()) {
        Ok(datastore) => datastore,
        Err(_) => {
            let datastore = MemoryDatastore::create_msgpack(path).unwrap();
            datastore
        }
    }
}

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
    fn create_empty_datastore() {
        let datastore: MemoryDatastore = get_clearhead_datastore("read_write.db");

        assert!(datastore.sync().is_ok())
    }

    #[test]
    fn read_empty_data_store() {
        let datastore: MemoryDatastore = get_clearhead_datastore("test_read.db");

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
