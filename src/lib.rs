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
            id: Uuid::parse_str(&record[3]).unwrap(),
            name: record[0].to_string(),
            completed: FromStr::from_str(&record[2])?,
            priority: parse_priority(&record[1])?,
        };
        import_list.tasks.push_back(new_task);
    }
    Ok(import_list)
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
    pub fn load_csv(&mut self, file_name: &str) -> Result<String, Box<dyn Error>> {
        let pathbuf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("data")
            .join(file_name);
        let mut wtr: Writer<std::fs::File> = Writer::from_path(pathbuf)?;
        for index in 0..=self.tasks.len() - 1 {
            wtr.serialize::<_>(&self.tasks[index])?;
        }
        Ok("Successfully Saved Tasks Into CSV".to_string())
    }

    pub fn create_task(self) -> Self {
        let new_task = create_default_task();
        let mut new_list = self.clone();
        new_list.tasks.push_back(new_task);
        return new_list
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

    pub fn remove_task(&mut self, index: usize) -> Result<String, Box<dyn Error>> {
        if index < self.tasks.len() {
            let removal_task = self.tasks.remove(index);
            return Ok(format!("Successfully Removed {}", &removal_task.name));
        } else {
            return Err(Box::new(OtherError::new(
                ErrorKind::Other,
                "No Task in that position",
            )));
        }
    }

    pub fn rename_task(self, index: usize, new_name: String) -> Result<TaskList, Box<dyn Error>> {
        let renaming_target = self.check_index_bounds(&index);
        match renaming_target {
            Ok(()) => Ok(TaskList {
                tasks: self
                    .tasks
                    .update(index, self.tasks[index].clone().rename(&new_name)),
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

    pub fn check_index_bounds(&self, index: &usize) -> Result<(), Box<dyn Error>> {
        if index < &self.tasks.len() {
            Ok(())
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

    pub fn mark_complete(&mut self) -> Result<String, Box<dyn Error>> {
        if self.completed == false {
            self.completed = true;
            return Ok(format!("completed Task: {}", self.name));
        } else {
            return Err(Box::new(OtherError::new(
                ErrorKind::Other,
                "Task is already completed",
            )));
        }
    }

    pub fn change_priority(&mut self, new_priority: &str) -> Result<String, Box<dyn Error>> {
        let new_pri: PriEnum = parse_priority(new_priority)?;
        self.priority = new_pri.clone();
        return Ok(format!(
            "changed Task: {name} priority changed to {new}",
            name = self.name,
            new = self.priority
        ));
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

    impl TaskList {
        fn create_nil_task(&mut self) {
            let new_task: Task = Task {
                id: Uuid::nil(),
                ..Default::default()
            };
            self.tasks.push_back(new_task);
        }
    }

    mod task_list_tests {
        use super::*;

        #[test]
        fn create_task_list_test() {
            let test_task_list = create_task_list();
            assert_eq!(test_task_list, TaskList { tasks: vector![] });
        }

        #[test]
        fn load_from_csv_bad_file_test() {
            let error = load_tasks_from_csv("bad_file").unwrap_err();
            assert_eq!(error.to_string(), "No such file or directory (os error 2)");
        }

        #[test]
        fn load_from_csv_bad_completion_status_test() {
            let mut test_task_list = create_task_list();
            let error = load_tasks_from_csv("bad_completion_status.csv").unwrap_err();
            assert_eq!(
                error.to_string(),
                "provided string was not `true` or `false`"
            );
        }

        #[test]
        fn load_from_csv_bad_priority_test() {
            let error = load_tasks_from_csv("bad_priority_test.csv")
                .unwrap_err();
            assert_eq!(error.to_string(), "invalid priority");
        }

        #[test]
        fn load_from_csv_sucessful_test() {
            let mut test_task_list = load_tasks_from_csv("successful_import_test.csv")
                .unwrap();
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
            let mut test_task_list = create_task_list();
            test_task_list.create_nil_task();
            test_task_list.tasks[0].clone().rename(&"test csv task".to_string());
            test_task_list.load_csv("successful_export_test.csv")?;
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
                        "test csv task",
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
        fn task_creation_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = create_task_list();
            test_task_list.create_task();
            let test_task = &test_task_list.tasks[0];
            assert!(test_task.name == "Default Task");
            assert!(test_task.completed == false);
            assert!(test_task.priority == PriEnum::Optional);
            assert!(&test_task_list.tasks[0] == test_task);
            return Ok(());
        }

        #[test]
        fn task_creation_new_id_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = create_task_list();
            test_task_list.create_task();
            test_task_list.create_task();
            let new_id_test_task = &test_task_list.tasks[1];
            assert!(new_id_test_task.id != test_task_list.tasks[0].id);
            return Ok(());
        }

        #[test]
        fn task_successful_search_by_index_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = create_task_list();
            test_task_list.create_nil_task();
            let successful_bounds_check = test_task_list.check_index_bounds(&0).unwrap();
            assert!(successful_bounds_check == ());
            return Ok(());
        }

        #[test]
        fn task_failed_search_by_index_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = create_task_list();
            let failed_bounds_check = test_task_list.check_index_bounds(&0);
            assert!(failed_bounds_check.unwrap_err().to_string() == "No Task at given Index");
            return Ok(());
        }

        #[test]
        fn task_successful_search_by_id_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = create_task_list();
            test_task_list.create_nil_task();
            let test_search_task = test_task_list
                .select_task_by_id(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
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
            let mut test_task_list = create_task_list();
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
            let mut test_task_list = create_task_list();
            test_task_list.create_task();

            let mut good_result = Vec::new();
            let success = test_task_list.print_task_list(&mut good_result).unwrap();

            let good_result_lines: Vec<&str> = std::str::from_utf8(&good_result)
                .unwrap()
                .split_inclusive('\n')
                .collect();
            let mut good_result_without_id: Vec<String> = Vec::new();
            for line in good_result_lines {
                let mut words: Vec<&str> = line.split(",").collect();
                words.remove(words.len() - 1);
                let joined: String = words.join(",").to_string();
                good_result_without_id.push(joined);
            }

            assert_eq!(
                format!("{}{}", good_result_without_id.join("\n"), "\n"),
                "name,priority,completed\nDefault Task,Optional,false\n"
            );
            assert_eq!(success, "End of List");
            return Ok(());
        }

        #[test]
        fn task_removal_fail_test() {
            let mut test_task_list = create_task_list();
            let error = test_task_list.remove_task(1).unwrap_err();
            assert_eq!(error.to_string(), "No Task in that position");
        }

        #[test]
        fn task_removal_successful_test() {
            let mut test_task_list = create_task_list();
            test_task_list.create_task();
            let good_result = test_task_list.remove_task(0).unwrap();
            assert_eq!(good_result.to_string(), "Successfully Removed Default Task");
        }
    }

    mod task_tests {
        use super::*;

        #[test]
        fn task_default_creation_test() -> Result<(), Box<dyn Error>> {
            let test_task = create_default_task();
            assert!(test_task.name == "Default Task".to_string());
            assert!(test_task.priority == PriEnum::Optional);
            assert!(test_task.completed == false);
            return Ok(());
        }

        #[test]
        fn task_rename_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = create_task_list();
            test_task_list.create_task();
            let test_task = test_task_list.tasks[0].clone().rename(&"Changed Name".to_string());
            assert!(test_task.name == "Changed Name");
            return Ok(());
        }

        #[test]
        fn task_completion_successful_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = create_task_list();
            test_task_list.create_task();
            let test_task = &mut test_task_list.tasks[0];
            test_task.mark_complete()?;
            assert!(test_task.completed == true);
            return Ok(());
        }

        #[test]
        fn task_completion_fail_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = create_task_list();
            test_task_list.create_task();
            let test_task = &mut test_task_list.tasks[0];
            test_task.mark_complete()?;
            let failure = test_task.mark_complete().unwrap_err();
            assert_eq!(failure.to_string(), "Task is already completed");
            return Ok(());
        }

        #[test]
        fn task_reprioritize_failure_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = create_task_list();
            test_task_list.create_task();
            let test_task = &mut test_task_list.tasks[0];
            let error = test_task.change_priority("6").unwrap_err();
            assert_eq!(error.to_string(), "invalid priority");
            return Ok(());
        }

        #[test]
        fn task_successful_reprioritize_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = create_task_list();
            test_task_list.create_task();
            let test_task = &mut test_task_list.tasks[0];
            test_task.change_priority("4")?;
            assert!(test_task.priority == PriEnum::Low);
            test_task.change_priority("3")?;
            assert!(test_task.priority == PriEnum::Medium);
            test_task.change_priority("2")?;
            assert!(test_task.priority == PriEnum::High);
            test_task.change_priority("1")?;
            assert!(test_task.priority == PriEnum::Critical);
            return Ok(());
        }
    }
}
