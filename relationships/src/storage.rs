use crate::Relationship;
use crate::RelationshipListManagement;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::{error::Error, path::Path};

use im::Vector;

pub trait CsvStorage {
    type L: RelationshipListManagement;
    fn write_to_csv(&self, file_path: &Path) -> Result<(), Box<dyn Error>>;
    fn read_from_csv(file_path: &Path) -> Result<Self::L, Box<dyn Error>>;
}

pub trait JSONStorage {
    type L: RelationshipListManagement;
    fn write_to_json(&self, file_path: &Path, pretty_print: bool) -> Result<(), Box<dyn Error>>;
}

impl CsvStorage for Vector<Relationship> {
    type L = Vector<Relationship>;
    fn write_to_csv(&self, file_path: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = csv::Writer::from_path(file_path)?;
        Ok(file.serialize(self)?)
    }

    fn read_from_csv(file_path: &Path) -> Result<Self::L, Box<dyn Error>> {
        let mut file_reader = csv::Reader::from_path(file_path)?;
        let mut relationship_list: Vector<Relationship> = Vector::new();

        for record in file_reader.deserialize() {
            let deserialized_relationship: Relationship = record?;
            relationship_list.push_back(deserialized_relationship)
        }

        Ok(relationship_list)
    }
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
}

#[cfg(test)]
mod test {
    use std::{fs, io::Read, path::Path};

    use super::*;
    use crate::tests::add_nil_relationship_to_vector;

    #[test]
    fn successful_relationship_list_save() {
        let empty_list: Vector<Relationship> = Vector::new();
        let single_list = add_nil_relationship_to_vector(empty_list);
        let file_path = Path::new("data/test_relationship.csv");
        let mut file_contents = String::new();

        single_list.write_to_csv(file_path).unwrap();

        fs::File::open(file_path)
            .unwrap()
            .read_to_string(&mut file_contents)
            .unwrap();

        assert_eq!(file_contents,"id,variant,participant_1,participant_2\n00000000-0000-0000-0000-000000000000,Undirected,00000000-0000-0000-0000-000000000000,00000000-0000-0000-0000-000000000000\n")
    }

    #[test]
    fn failed_relationship_list_save() {
        let empty_list: Vector<Relationship> = Vector::new();
        let single_list = add_nil_relationship_to_vector(empty_list);
        let file_path = Path::new("bad_folder/bad_file");

        let directory_error = single_list.write_to_csv(file_path).unwrap_err();

        assert_eq!(
            directory_error.to_string(),
            "No such file or directory (os error 2)"
        )
    }
    #[test]
    fn successful_relationship_list_read() {
        //Need to be sure and implement or delete this after the JSON variant is completed
        //let file_path = Path::new("data/successful_relationship_list_read_test.csv");

        //let relationship_list: Vector<Relationship> = Vector::read_from_csv(file_path).unwrap();
    }

    #[test]
    fn successfully_read_pretty_json() {
        let empty_list: Vector<Relationship> = Vector::new();
        let single_list = add_nil_relationship_to_vector(empty_list);
        let file_path = Path::new("data/test_pretty_relationship.json");
        let mut file_contents = String::new();

        single_list.write_to_json(file_path, true).unwrap();

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
    fn successfully_read_json() {
        let empty_list: Vector<Relationship> = Vector::new();
        let single_list = add_nil_relationship_to_vector(empty_list);
        let file_path = Path::new("data/test_relationship.json");
        let mut file_contents = String::new();

        single_list.write_to_json(file_path, false).unwrap();

        fs::File::open(file_path)
            .unwrap()
            .read_to_string(&mut file_contents)
            .unwrap();

        assert_eq!(
            file_contents,
            "[{\"id\":\"00000000-0000-0000-0000-000000000000\",\"variant\":{\"Related\":\"Undirected\"},\"participant_1\":\"00000000-0000-0000-0000-000000000000\",\"participant_2\":\"00000000-0000-0000-0000-000000000000\"}]"
        )
    }
}
