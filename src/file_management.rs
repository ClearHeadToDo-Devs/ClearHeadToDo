use crate::Action;

use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;
use xdg::*;

use std::io::Error;

pub fn save_action_list(list: Vec<Action>) -> Result<(), Error> {
    let xdg_dirs = BaseDirectories::with_prefix("myapp").unwrap();

    let data_path = xdg_dirs
        .place_data_file("actions.json")
        .expect("cannot create configuration directory");
    let mut data_file = File::create(data_path)?;
    write!(&mut data_file, "{:?}", list)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::action_builder::ActionBuilder;

    use super::*;
    use crate::priority::Priority;
    use serde_json;
    use serde_test::*;
    use uuid::Uuid;

    use std::{fs::File, io::Read};

    #[test]
    fn create_file_in_data_dir() {
        let test_list = vec![ActionBuilder::default().build()];

        save_action_list(test_list);
        let file_path = BaseDirectories::with_prefix("clearhead")
            .unwrap()
            .find_data_file("actions.json")
            .expect("couldn't find application data");

        let mut file_contents = String::new();

        File::open(file_path)
            .unwrap()
            .read_to_string(&mut file_contents)
            .unwrap();
        assert!(file_contents == "{}")
    }
}
