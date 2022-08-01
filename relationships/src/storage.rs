use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::Write, path::Path};

use crate::Relationship;
use im::Vector;
pub trait CsvStorage {
    fn write_to_csv(&self, file_path: &Path) -> Result<(), Box<dyn Error>>;
}

impl CsvStorage for Vector<Relationship> {
    fn write_to_csv(&self, file_path: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = csv::Writer::from_path(file_path)?;
        file.serialize(self)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{fs, io::Read, path::Path};

    use super::*;
    use crate::tests::add_nil_relationship_to_vector;

    #[test]
    fn load_relationship_list() {
        let empty_list: Vector<Relationship> = Vector::new();
        let single_list = add_nil_relationship_to_vector(empty_list);
        let file_path = Path::new("../data/test_relationship.csv");
        let mut file_contents = String::new();

        single_list.write_to_csv(file_path).unwrap();

        fs::File::open(file_path)
            .unwrap()
            .read_to_string(&mut file_contents)
            .unwrap();
        assert_eq!(file_contents,"id,variant,participant_1,participant_2\n00000000-0000-0000-0000-000000000000,Undirected,00000000-0000-0000-0000-000000000000,00000000-0000-0000-0000-000000000000\n")
    }
}
