use crate::Action;

use csv::Reader;
use csv::Writer;
use im::Vector;
use std::error::Error;
use std::{env, path::PathBuf};

pub fn load_action_from_csv(file_name: &str) -> Result<im::Vector<Action>, Box<dyn Error>> {
    let mut rdr: Reader<std::fs::File> = create_file_reader_from_data_folder(file_name)?;
    let mut action_list = Vector::new();

    for record in rdr.deserialize() {
        let action: Action = record?;
        action_list.push_back(action);
    }

    Ok(action_list)
}

pub fn load_csv_with_action_data(
    task_list: &im::Vector<Action>,
    file_name: &str,
) -> Result<(), Box<dyn Error>> {
    let mut csv_writer: Writer<std::fs::File> = create_file_writer_to_data_folder(file_name)?;

    csv_writer.serialize(task_list)?;
    Ok(())
}

fn create_file_reader_from_data_folder(
    file_name: &str,
) -> Result<Reader<std::fs::File>, Box<dyn Error>> {
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(file_name);

    let file_reader = Reader::from_path(file_path)?;
    Ok(file_reader)
}

fn create_file_writer_to_data_folder(
    file_name: &str,
) -> Result<Writer<std::fs::File>, Box<dyn Error>> {
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(file_name);

    let file_writer = Writer::from_path(file_path)?;
    Ok(file_writer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::add_nil_action;
    use im::vector;
    use crate::Priority;
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn load_from_csv_sucessful() {
        let test_task_list = load_action_from_csv("successful_import_test.csv").unwrap();
        let test_task = &test_task_list[0];

        assert!(test_task.id == Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
        assert!(test_task.name == "test csv task");
        assert!(test_task.completed == false);
        assert!(test_task.priority == Priority::Optional);
    }

    #[test]
    fn load_action_data_to_csv_successful() -> Result<(), Box<dyn Error>> {
        let empty_task_list = vector!();
        let single_nil_task_list = add_nil_action(empty_task_list);

        load_csv_with_action_data(&single_nil_task_list, "successful_export_test.csv")?;
        let rdr = Reader::from_path(
            env::current_dir()?
                .join("data")
                .join("successful_export_test.csv")
                .as_path(),
        )?;

        let mut iter = rdr.into_records();
        if let Some(result) = iter.next() {
            let record = result?;
            assert_eq!(
                record,
                vec![
                    "00000000-0000-0000-0000-000000000000",
                    "Default Action",
                    "false",
                    "Optional"
                ]
            );
        } else {
            return Ok(());
        }
        return Ok(());
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn load_from_csv_bad_file() {
        let error = load_action_from_csv("bad_file").unwrap_err();
        assert_eq!(error.to_string(), "No such file or directory (os error 2)");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn load_from_csv_bad_file() {
        let error = load_action_from_csv("bad_file").unwrap_err();
        assert_eq!(
            error.to_string(),
            "The system cannot find the file specified. (os error 2)"
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn load_from_csv_bad_completion_status() {
        let error = load_action_from_csv("bad_completion_status.csv").unwrap_err();
        assert_eq!(
            error.to_string(),
            "CSV deserialize error: record 1 (line: 2, byte: 26): missing field `completed`"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn load_from_csv_bad_completion_status() {
        let error = load_action_from_csv("bad_completion_status.csv").unwrap_err();
        assert_eq!(
            error.to_string(),
            "CSV deserialize error: record 1 (line: 1, byte: 26): missing field `completed`"
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn load_from_csv_bad_priority() {
        let error = load_action_from_csv("bad_priority_test.csv").unwrap_err();
        assert_eq!(
            error.to_string(),
            "CSV deserialize error: record 1 (line: 2, byte: 28): unknown variant `bad priority`, expected one of `Critical`, `High`, `Medium`, `Low`, `Optional`"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn load_from_csv_bad_priority() {
        let error = load_action_from_csv("bad_priority_test.csv").unwrap_err();
        assert_eq!(
            error.to_string(),
            "CSV deserialize error: record 1 (line: 1, byte: 28): unknown variant `bad priority`, expected one of `Critical`, `High`, `Medium`, `Low`, `Optional`"
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn load_from_csv_bad_id() {
        let error = load_action_from_csv("bad_id_test.csv").unwrap_err();
        assert_eq!(
            error.to_string(),
            "CSV deserialize error: record 1 (line: 2, byte: 29): missing field `id`"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn load_from_csv_bad_id() {
        let error = load_action_from_csv("bad_id_test.csv").unwrap_err();
        assert_eq!(
            error.to_string(),
            "CSV deserialize error: record 1 (line: 1, byte: 28): missing field `id`"
        );
    }
}
