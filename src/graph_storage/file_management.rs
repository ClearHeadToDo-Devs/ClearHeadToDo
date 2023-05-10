use std::{io::ErrorKind, path::PathBuf};

use dirs::*;
use indradb::MemoryDatastore;

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
    let clearhead_directories = get_or_create_clearhead_data_directory().unwrap();

    return clearhead_directories.join(file_name);
}
fn get_or_create_clearhead_data_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let clearhead_directories: PathBuf = data_dir()
        .expect("Unable to return your data directory path for some reason")
        .join("clearhead");

    std::fs::create_dir_all(clearhead_directories.clone())?;

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
        let directory = get_or_create_clearhead_data_directory().unwrap();

        assert!(directory.to_str().unwrap().contains("clearhead"));
    }
}
