use crate::Action;

use std::io::Write;
use std::path::PathBuf;
use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;
use xdg::*;

use std::io::Error;

fn get_clearhead_database_path() -> PathBuf {
    let clearhead_directories = get_clearhead_base_directories().unwrap();

    return clearhead_directories.get_data_home().join("clearhead.db");
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
        let path = get_clearhead_database_path();

        let datastore: MemoryDatastore = MemoryDatastore::create_msgpack(path).unwrap();

        assert!(datastore.sync().is_ok())
    }

    #[test]
    fn get_datastore_location() {
        let path = get_clearhead_database_path();

        assert!(path.to_str().unwrap().contains("clearhead.db"));
    }

    #[test]
    fn create_test_data_file() {
        let directory = get_clearhead_base_directories().unwrap().get_data_home();

        let test_file = File::create(directory.join("test.txt"))
            .unwrap()
            .write("test content".as_bytes());

        assert!(test_file.is_ok())
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
