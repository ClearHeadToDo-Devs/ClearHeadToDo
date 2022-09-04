use crate::Relationship;
use crate::RelationshipListManagement;

use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::{error::Error, path::Path};

use im::Vector;

pub trait JSONStorage {
    type L: RelationshipListManagement;
    fn write_to_json(&self, file_path: &Path, pretty_print: bool) -> Result<(), Box<dyn Error>>;
    fn read_from_json(file_path: &Path) -> Result<Self::L, Box<dyn Error>>;
}

impl JSONStorage for Vector<Relationship> {
    type L = Vector<Relationship>;
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

    fn read_from_json(file_path: &Path) -> Result<Self::L, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let file_reader = BufReader::new(file);

        let new_list = serde_json::from_reader(file_reader)?;

        Ok(new_list)
    }
}

#[cfg(test)]
mod test {
    use std::{fs, io::Read, path::Path};


    use super::*;

    use crate::tests::create_vector_with_nill_relationship;

    #[test]
    fn successfully_write_pretty_json() {
        let list = create_vector_with_nill_relationship();

        let file_path = Path::new("data/test_pretty_relationship.json");
        let mut file_contents = String::new();

        list.write_to_json(file_path, true).unwrap();

        fs::File::open(file_path)
            .unwrap()
            .read_to_string(&mut file_contents)
            .unwrap();

        assert_eq!(
            file_contents,
            "[
  {
    \"id\": \"00000000-0000-0000-0000-000000000000\",
    \"variant\": {
      \"Related\": \"Undirected\"
    },
    \"participant_1\": \"00000000-0000-0000-0000-000000000000\",
    \"participant_2\": \"00000000-0000-0000-0000-000000000000\"
  }
]"
        )
    }

    #[test]
    fn successfully_write_json() {
        let list = create_vector_with_nill_relationship();

        let file_path = Path::new("data/test_relationship.json");
        let mut file_contents = String::new();

        list.write_to_json(file_path, false).unwrap();

        fs::File::open(file_path)
            .unwrap()
            .read_to_string(&mut file_contents)
            .unwrap();

        assert_eq!(
            file_contents,
            "[{\"id\":\"00000000-0000-0000-0000-000000000000\",\"variant\":{\"Related\":\"Undirected\"},\"participant_1\":\"00000000-0000-0000-0000-000000000000\",\"participant_2\":\"00000000-0000-0000-0000-000000000000\"}]"
        )
    }

    #[test]
    fn successfully_read_json() {
        let file_list =
            Vector::<Relationship>::read_from_json(Path::new("data/test_read_relationship.json"))
                .unwrap();

        assert_eq!(file_list, create_vector_with_nill_relationship());
    }

    #[test]
    fn successfully_read_pretty_json() {
        let file_list = Vector::<Relationship>::read_from_json(Path::new(
            "data/test_read_pretty_relationship.json",
        ))
        .unwrap();

        assert_eq!(file_list, create_vector_with_nill_relationship())
    }
}
