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

#[derive(Debug, PartialEq, Clone)]
pub struct TaskList {
    pub tasks: im::Vector<Task>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub completed: bool,
    pub priority: PriEnum,
}

pub fn create_task_list() -> TaskList {
    return TaskList { tasks: vector![] };
}

pub fn create_default_task() -> Task {
    Task {
        ..Default::default()
    }
}

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

#[repr(u8)]
#[derive(AltSerialize, Copy, Clone, PartialEq, Debug)]
pub enum PriEnum {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Optional = 5,
}

impl TaskList {
    pub fn create_task(self) -> Self {
        let new_task = create_default_task();
        let mut new_list = self.clone();
        new_list.tasks.push_back(new_task);
        return new_list;
    }

    pub fn print_task_list(
        &self,
        mut writer: impl std::io::Write,
    ) -> Result<String, Box<dyn Error>> {
        if self.tasks.is_empty() == true {
            return Err(Box::new(OtherError::new(ErrorKind::Other, "list is empty")));
        } else {
            writeln!(writer, "name,priority,completed,ID")?;
            for task in &self.tasks {
                writeln!(
                    writer,
                    "{name},{priority},{completed},{id}",
                    name = task.name,
                    priority = task.priority,
                    completed = task.completed,
                    id = task.id
                )?;
            }
        }
        Ok("End of List".to_string())
    }

    pub fn remove_task(self, index: usize) -> Result<TaskList, Box<dyn Error>> {
        let index_check_result = self.check_index_bounds(index);
        match index_check_result {
            Ok(checked_index) => {
                let (mut left_side, mut right_side) = self.tasks.split_at(checked_index);
                right_side.pop_front().unwrap();
                left_side.append(right_side);
                Ok(TaskList { tasks: left_side })
            }
            Err(error) => Err(error),
        }
    }

    pub fn rename_task(self, index: usize, new_name: String) -> Result<TaskList, Box<dyn Error>> {
        let index_bounds_result = self.check_index_bounds(index);
        match index_bounds_result {
            Ok(checked_index) => Ok(TaskList {
                tasks: self.tasks.update(
                    checked_index,
                    self.tasks[checked_index].clone().rename(&new_name),
                ),
            }),
            Err(error) => Err(error),
        }
    }

    pub fn complete_task(self, index: usize) -> Result<TaskList, Box<dyn Error>> {
        let index_bounds_result = self.check_index_bounds(index);
        match index_bounds_result {
            Ok(checked_index) => Ok(TaskList {
                tasks: self.tasks.update(
                    checked_index,
                    self.tasks[checked_index].clone().mark_complete()?,
                ),
            }),
            Err(error) => Err(error),
        }
    }

    pub fn change_task_priority(
        self,
        index: usize,
        new_priority: String,
    ) -> Result<TaskList, Box<dyn Error>> {
        let index_bounds_result = self.check_index_bounds(index);
        match index_bounds_result {
            Ok(checked_index) => Ok(TaskList {
                tasks: self.tasks.update(
                    checked_index,
                    self.tasks[checked_index]
                        .clone()
                        .change_priority(&new_priority)?,
                ),
            }),
            Err(error) => Err(error),
        }
    }

    pub fn select_task_by_id(self, id: Uuid) -> Result<Task, Box<dyn Error>> {
        let search_task = self.tasks.into_iter().find(|tasks| tasks.id == id);
        match search_task {
            Some(task) => return Ok(task),
            None => {
                return Err(Box::new(OtherError::new(
                    ErrorKind::Other,
                    "No Task with given ID",
                )))
            }
        }
    }

    pub fn check_index_bounds(&self, index: usize) -> Result<usize, Box<dyn Error>> {
        if index < self.tasks.len() {
            Ok(index)
        } else {
            Err(Box::new(OtherError::new(
                ErrorKind::Other,
                "No Task in that position",
            )))
        }
    }
}

impl Task {
    pub fn rename(self, new_task_name: &String) -> Task {
        return Task {
            name: new_task_name.to_owned(),
            id: self.id,
            priority: self.priority,
            completed: self.completed,
        };
    }

    pub fn mark_complete(self) -> Result<Task, Box<dyn Error>> {
        if self.completed == false {
            return Ok(Task {
                id: self.id,
                name: self.name,
                priority: self.priority,
                completed: true,
            });
        } else {
            return Err(Box::new(OtherError::new(
                ErrorKind::Other,
                "Task is already completed",
            )));
        }
    }

    pub fn change_priority(self, new_priority: &str) -> Result<Task, Box<dyn Error>> {
        let new_pri: PriEnum = parse_priority(new_priority)?;
        return Ok(Task {
            name: self.name,
            priority: new_pri,
            id: self.id,
            completed: self.completed,
        });
    }
}

pub fn parse_priority(expr: &str) -> Result<PriEnum, Box<dyn Error>> {
    match expr.to_ascii_lowercase().trim() {
        "1" | "critical" | "crit" | "c" => Ok(PriEnum::Critical),
        "2" | "high" | "hi" | "h" => Ok(PriEnum::High),
        "3" | "medium" | "med" | "m" => Ok(PriEnum::Medium),
        "4" | "low" | "lo" | "l" => Ok(PriEnum::Low),
        "5" | "optional" | "opt" | "o" => Ok(PriEnum::Optional),
        "" => Ok(PriEnum::Optional), //defaults to this
        _ => Err(Box::new(OtherError::new(
            ErrorKind::Other,
            "invalid priority",
        ))),
    }
}

impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Task", 4)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("priority", &self.priority)?;
        s.serialize_field("completed", &self.completed)?;
        s.serialize_field("id", &self.id)?;
        s.end()
    }
}

impl fmt::Display for PriEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable: &str = match *self {
            PriEnum::Critical => "Critical",
            PriEnum::High => "High",
            PriEnum::Medium => "Medium",
            PriEnum::Low => "Low",
            PriEnum::Optional => "Optional",
        };
        write!(f, "{}", printable)
    }
}

impl Default for PriEnum {
    fn default() -> Self {
        PriEnum::Optional
    }
}

impl Default for Task {
    fn default() -> Task {
        Task {
            id: Uuid::new_v4(),
            name: "Default Task".to_string(),
            completed: false,
            priority: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_nil_task() -> Task {
        Task {
            id: Uuid::nil(),
            ..Default::default()
        }
    }

    impl TaskList {
        fn create_nil_task(self) -> Self {
            let mut new_list = self.clone();
            let new_task: Task = create_nil_task();
            new_list.tasks.push_back(new_task);
            return new_list;
        }
    }

    mod storage {
        use super::*;

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
            let single_nil_task_list = empty_task_list.create_nil_task();
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

    mod task_list {
        use super::*;

        #[test]
        fn task_list_creation_test() {
            let test_task_list = create_task_list();
            assert_eq!(test_task_list, TaskList { tasks: vector![] });
        }

        #[test]
        fn child_task_creation_test() -> Result<(), Box<dyn Error>> {
            let empty_task_list = create_task_list();
            let single_task_list = empty_task_list.create_task();
            let test_task = &single_task_list.tasks[0];
            assert!(test_task.name == "Default Task");
            assert!(test_task.completed == false);
            assert!(test_task.priority == PriEnum::Optional);
            assert!(&single_task_list.tasks[0] == test_task);
            return Ok(());
        }

        #[test]
        fn successful_search_tasks_by_index_test() -> Result<(), Box<dyn Error>> {
            let empty_list = create_task_list();
            let single_nil_task_list = empty_list.create_task();
            let successful_bounds_check = single_nil_task_list.check_index_bounds(0).unwrap();
            assert!(successful_bounds_check == 0);
            return Ok(());
        }

        #[test]
        fn task_failed_search_by_index_test() -> Result<(), Box<dyn Error>> {
            let test_task_list = create_task_list();
            let failed_bounds_check = test_task_list
                .check_index_bounds(0)
                .unwrap_err()
                .to_string();
            assert_eq!(failed_bounds_check, "No Task in that position".to_string());
            return Ok(());
        }

        #[test]
        fn task_successful_search_by_id_test() -> Result<(), Box<dyn Error>> {
            let empty_list = create_task_list();
            let single_nil_task_list = empty_list.create_nil_task();
            let test_search_task = single_nil_task_list.select_task_by_id(Uuid::nil());
            assert!(
                test_search_task.unwrap()
                    == Task {
                        id: Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
                        name: String::from("Default Task"),
                        completed: false,
                        priority: PriEnum::Optional
                    }
            );

            return Ok(());
        }

        #[test]
        fn task_failed_search_by_id_test() -> Result<(), Box<dyn Error>> {
            let test_task_list = create_task_list();
            let test_search_task = test_task_list
                .select_task_by_id(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
            assert!(test_search_task.unwrap_err().to_string() == "No Task with given ID");
            return Ok(());
        }

        #[test]
        fn task_print_fail_test() {
            let test_task_list = create_task_list();
            let mut bad_result = Vec::new();
            let error = test_task_list.print_task_list(&mut bad_result).unwrap_err();
            assert_eq!(error.to_string(), "list is empty");
        }

        #[test]
        fn task_print_successful_test() -> Result<(), Box<dyn Error>> {
            let empty_task_list = create_task_list();
            let single_task_list = empty_task_list.create_nil_task();

            let mut good_result = Vec::new();
            let success = single_task_list.print_task_list(&mut good_result).unwrap();

            assert_eq!(
                format!("{}", std::str::from_utf8(&good_result).unwrap().to_string()),
                "name,priority,completed,ID\nDefault Task,Optional,false,00000000-0000-0000-0000-000000000000\n"
            );
            assert_eq!(success, "End of List");
            return Ok(());
        }

        #[test]
        fn task_removal_fail_test() {
            let test_task_list = create_task_list();
            let error = test_task_list.remove_task(1).unwrap_err();
            assert_eq!(error.to_string(), "No Task in that position");
        }

        #[test]
        fn task_removal_successful_test() {
            let empty_task_list = create_task_list();
            let single_task_list = empty_task_list.create_task();
            let good_result = single_task_list.remove_task(0).unwrap();
            assert!(good_result.tasks.is_empty());
        }
    }

    mod task {
        use super::*;

        #[test]
        fn default_creation_test() {
            let test_task = create_nil_task();
            assert!(test_task.name == "Default Task".to_string());
            assert!(test_task.priority == PriEnum::Optional);
            assert!(test_task.completed == false);
            assert!(test_task.id.to_string() == "00000000-0000-0000-0000-000000000000".to_string());
        }

        #[test]
        fn task_creation_unique_id_test() {
            let first_test_task = create_default_task();
            let second_test_task = create_default_task();

            assert!(first_test_task.id != second_test_task.id);
        }

        #[test]
        fn rename_test() {
            let test_task = create_default_task();
            let renamed_task = test_task.rename(&"Changed Name".to_string());

            assert!(renamed_task.name == "Changed Name");
        }

        #[test]
        fn successful_completion_test() -> Result<(), Box<dyn Error>> {
            let test_task = create_default_task();
            let test_successful_completion_task = test_task.mark_complete()?;

            assert!(test_successful_completion_task.completed == true);
            return Ok(());
        }

        #[test]
        fn failing_completion_test() -> Result<(), Box<dyn Error>> {
            let test_task = create_default_task();
            let test_first_completion_task = test_task.mark_complete()?;
            let failure = test_first_completion_task.mark_complete().unwrap_err();
            assert_eq!(failure.to_string(), "Task is already completed");
            return Ok(());
        }

        #[test]
        fn failing_reprioritize_test() -> Result<(), Box<dyn Error>> {
            let test_task = create_default_task();
            let error = test_task.change_priority("6").unwrap_err();
            assert_eq!(error.to_string(), "invalid priority");
            return Ok(());
        }

        #[test]
        fn successful_reprioritize_test() -> Result<(), Box<dyn Error>> {
            let priority_5_test_task = create_default_task();

            let priority_4_test_task = priority_5_test_task.change_priority("4")?;
            assert!(priority_4_test_task.priority == PriEnum::Low);

            let priority_3_test_task = priority_4_test_task.change_priority("3")?;
            assert!(priority_3_test_task.priority == PriEnum::Medium);

            let priority_2_test_task = priority_3_test_task.change_priority("2")?;
            assert!(priority_2_test_task.priority == PriEnum::High);

            let priority_1_test_task = priority_2_test_task.change_priority("1")?;
            assert!(priority_1_test_task.priority == PriEnum::Critical);

            return Ok(());
        }
    }
}
