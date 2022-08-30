use std::fs::File;
use crate::functionality::ClearHeadApp;

use std::io::{BufReader, BufWriter, Write};
use std::{error::Error, path::Path};

use serde_json;

pub trait JSONStorage {
    fn write_to_json(&self, file_path: &Path, pretty_print: bool) -> Result<(), Box<dyn Error>>;
    fn read_from_json(file_path: &Path) -> Result<ClearHeadApp, Box<dyn Error>>;
}

impl JSONStorage for ClearHeadApp {
    fn write_to_json(&self, file_path: &Path, pretty_print: bool) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_path)?;
        let mut file_writer = BufWriter::new(file);

        if pretty_print == true {
            serde_json::to_writer_pretty(&mut file_writer, &self)?;
            file_writer.flush()?;
        } else {
            serde_json::to_writer(&mut file_writer, &self)?;
        }

        Ok(())
    }

    fn read_from_json(file_path: &Path) -> Result<ClearHeadApp, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let file_reader = BufReader::new(file);

        let new_list = serde_json::from_reader(file_reader)?;

        Ok(new_list)
    }
}

#[cfg(test)]
mod tests{
    use crate::ClearHeadApp;
use std::path::Path;
    use std::fs::File;
    use std::io::Read;
    use super::JSONStorage;

    #[test]
    fn successfully_write_json_file() {
        let test_app: ClearHeadApp = Default::default();
        let file_path = Path::new("data/test_clearheadApp_write.json");
        let mut file_contents = String::new();

        test_app.write_to_json(file_path, true).unwrap();

        File::open(file_path)
            .unwrap()
            .read_to_string(&mut file_contents)
            .unwrap();

        assert_eq!(
            file_contents,
            "{\n  \"action_list\": [],\n  \"relationship_list\": []\n}"
        );
    

    }
   
    #[test]
    fn successfully_read_json_file() {
        let test_app: ClearHeadApp = Default::default();
        let file_path = Path::new("data/test_clearheadApp_read.json");

        let app_from_file: ClearHeadApp = ClearHeadApp::read_from_json(file_path).unwrap();

        assert_eq!(app_from_file, test_app);

    }
}
