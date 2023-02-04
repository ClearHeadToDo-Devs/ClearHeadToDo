use crate::Action;

use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;
use xdg::*;

use std::io::Error;

fn get_clearhead_base_directories() -> Result<BaseDirectories, BaseDirectoriesError> {
    let clearhead_directories = BaseDirectories::with_prefix("clearhead")?;

    Ok(clearhead_directories)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn return_clearhead_directory() {
        let directory = get_clearhead_base_directories().unwrap();

        assert!(directory.get_data_home().to_str().unwrap().contains("clearhead"));
    }

    #[test]
    fn return_clearhead_data_dir() {
        let clearhead_directories = BaseDirectories::with_prefix("clearhead");
    }
}
