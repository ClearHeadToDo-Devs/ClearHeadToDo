mod task;
use csv::Reader;
use csv::Writer;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Serialize as AltSerialize;
use std::error::Error;
use std::fmt;
use std::io::{Error as OtherError, ErrorKind};
use std::{env, path::PathBuf};
//use std::path::{Path, PathBuf};
use im::vector;
use std::str::FromStr;
use uuid::Uuid;
use clear_head_todo::TaskList;
use clear_head_todo::create_task_list;

pub fn load_tasks_from_csv(file_name: &str) -> Result<TaskList, Box<dyn Error>> {
    let mut import_list = create_task_list();
    let pathbuf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(file_name);
    let mut rdr: Reader<std::fs::File> = Reader::from_path(pathbuf)?;
    for result in rdr.records() {
        let record: csv::StringRecord = result?;
        let new_task: Task = Task {
            id: Uuid::parse_str(&record[3])?,
            name: record[0].to_string(),
            completed: FromStr::from_str(&record[2])?,
            priority: parse_priority(&record[1])?,
        };
        import_list.tasks.push_back(new_task);
    }
    Ok(import_list)
}

pub fn load_csv(task_list: &TaskList, file_name: &str) -> Result<String, Box<dyn Error>> {
    let pathbuf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(file_name);
    let mut wtr: Writer<std::fs::File> = Writer::from_path(pathbuf)?;
    for index in 0..=task_list.tasks.len() - 1 {
        wtr.serialize::<_>(&task_list.tasks[index])?;
    }
    Ok("Successfully Saved Tasks Into CSV".to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::add_nil_task;

    #[test]
    fn load_from_csv_sucessful_test() {
        let test_task_list = load_tasks_from_csv("successful_import_test.csv").unwrap();
        let test_task = &test_task_list.tasks[0];
        assert!(
            test_task.id == Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()
        );
        assert!(test_task.name == "test csv task");
        assert!(test_task.completed == false);
        assert!(test_task.priority == PriEnum::Optional);
    }

    #[test]
    fn load_to_csv_successful_test() -> Result<(), Box<dyn Error>> {
        let empty_task_list = create_task_list();
        let single_nil_task_list = empty_task_list.add_nil_task();
        single_nil_task_list.tasks[0]
            .clone()
            .rename(&"test csv task".to_string());
        load_csv(&single_nil_task_list, "successful_export_test.csv")?;
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
                    "Default Task",
                    "Optional",
                    "false",
                    "00000000-0000-0000-0000-000000000000"
                ]
            );
        } else {
            return Ok(());
        }
        return Ok(());
    }

    #[test]
    fn load_from_csv_bad_file_test() {
        let error = load_tasks_from_csv("bad_file").unwrap_err();
        assert_eq!(error.to_string(), "No such file or directory (os error 2)");
    }

    #[test]
    fn load_from_csv_bad_completion_status_test() {
        let error = load_tasks_from_csv("bad_completion_status.csv").unwrap_err();
        assert_eq!(
            error.to_string(),
            "provided string was not `true` or `false`"
        );
    }

    #[test]
    fn load_from_csv_bad_priority_test() {
        let error = load_tasks_from_csv("bad_priority_test.csv").unwrap_err();
        assert_eq!(error.to_string(), "invalid priority");
    }

    #[test]
    fn load_from_csv_bad_id_test() {
        let error = load_tasks_from_csv("bad_id_test.csv").unwrap_err();
        assert_eq!(
            error.to_string(),
            "invalid length: expected one of [36, 32], found 24"
        );
    }
}
