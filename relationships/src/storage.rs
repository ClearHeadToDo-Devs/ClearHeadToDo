use crate::Relationship;
use im::Vector;
pub trait CsvStorage {
    fn write_to_csv(&self) -> ();
}

impl CsvStorage for Vector<Relationship> {
    fn write_to_csv(&self) -> () {}
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

        single_list.write_to_csv();

        fs::File::open(file_path)
            .unwrap()
            .read_to_string(&mut file_contents)
            .unwrap();
        assert!(file_contents == "")
    }
}
