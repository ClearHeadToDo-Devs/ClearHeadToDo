use csv::Reader;
use csv::Writer;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Serialize as AltSerialize;
use std::error::Error;
use std::fmt;
use std::io::{Error as OtherError, ErrorKind};
use std::{env, path::PathBuf};
//use std::path::{Path, PathBuf};
use std::str::FromStr;

pub struct TaskList {
    pub tasks: Vec<Task>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Task {
    pub name: String,
    pub completed: bool,
    pub priority: PriEnum,
}

#[derive(PartialEq, Debug)]
#[repr(u8)]
#[derive(AltSerialize, Copy, Clone)]
pub enum PriEnum {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Optional = 5,
}

impl TaskList {
    //load tasks from either tasks.csv or testTasks.csv using the file_name
    pub fn load_tasks(&mut self, file_name: &str) -> Result<String, Box<dyn Error>> {
        let pathbuf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data").join(file_name);
        let mut rdr: Reader<std::fs::File> = Reader::from_path(pathbuf)?;
        for result in rdr.records() {
            let record: csv::StringRecord = result?;
            let new_task: Task = Task {
                name: record[0].to_string(),
                completed: FromStr::from_str(&record[2])?,
                priority: parse_priority(&record[1])?,
            };
            self.tasks.push(new_task);
        }
        Ok("Successfully Loaded Tasks into List".to_string())
    }

    pub fn load_csv(&mut self, file_name: &str) -> Result<String, Box<dyn Error>> {
        let pathbuf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data").join(file_name);
        let mut wtr: Writer<std::fs::File> = Writer::from_path(pathbuf)?;
        for index in 0..=self.tasks.len() - 1 {
            wtr.serialize::<_>(&self.tasks[index])?;
        }
        Ok("Successfully Saved Tasks Into CSV".to_string())
    }

    pub fn create_task(&mut self) -> Result<String, Box<dyn Error>> {
        let new_task: Task = Task {
            name: String::from("Test Task"),
            completed: false,
            priority: PriEnum::Optional,
        };
        let new_task_name: String = new_task.name.clone();
        self.tasks.push(new_task);
        return Ok(format!("Created new task named {}", new_task_name).to_string());
    }

    pub fn print_task_list(
        &self,
        mut writer: impl std::io::Write,
    ) -> Result<String, Box<dyn Error>> {
        if self.tasks.is_empty() == true {
            return Err(Box::new(OtherError::new(ErrorKind::Other, "list is empty")));
        } else {
            writeln!(writer, "index,name,priority,completed")?;
            for index in 0..=self.tasks.len() - 1 {
                writeln!(
                    writer,
                    "{index},{name},{priority},{completed}",
                    index = index,
                    name = self.tasks[index].name,
                    priority = self.tasks[index].priority,
                    completed = self.tasks[index].completed
                )?;
            }
        }
        Ok("End of List".to_string())
    }

    pub fn remove_task(&mut self, index: usize) -> Result<String, Box<dyn Error>> {
        if index < self.tasks.len() {
            let name: String = self.tasks[index].name.clone();
            self.tasks.remove(index);
            return Ok(format!("Successfully Removed Task {}", name).to_string());
        } else {
            return Err(Box::new(OtherError::new(
                ErrorKind::Other,
                "Invalid Index for Deletion",
            )));
        }
    }
} //end 'impl TaskList'

impl Task {
    pub fn rename_task(&mut self, new_task_name: &String) -> Result<String, Box<dyn Error>> {
        let old_name: String = self.name.clone();
        self.name = new_task_name.to_owned();
        return Ok(format!(
            "Task {old} renamed to {new}",
            old = old_name,
            new = self.name
        ));
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
        if self.priority == new_pri {
            return Err(Box::new(OtherError::new(
                ErrorKind::Other,
                "duplicate priority",
            )));
        } else {
            self.priority = new_pri.clone();
            return Ok(format!(
                "changed Task: {name} priority changed to {new}",
                name = self.name,
                new = self.priority
            ));
        }
    }
} //end 'impl Task'

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
        let mut s = serializer.serialize_struct("Task", 3)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("priority", &self.priority)?;
        s.serialize_field("completed", &self.completed)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    mod task_list_tests {
        use super::*;

        #[test]
        fn load_from_csv_bad_file_test() {
            let mut test_task_list = TaskList { tasks: vec![] };
            let error = test_task_list.load_tasks("bad_file").unwrap_err();
            assert_eq!(error.to_string(), "No such file or directory (os error 2)");
        }

        #[test]
        fn load_from_csv_bad_completion_status_test() {
            let mut test_task_list = TaskList { tasks: vec![] };
            let error = test_task_list
                .load_tasks("bad_completion_status.csv")
                .unwrap_err();
            assert_eq!(
                error.to_string(),
                "provided string was not `true` or `false`"
            );
        }

        #[test]
        fn load_from_csv_bad_priority_test() {
            let mut test_task_list = TaskList { tasks: vec![] };
            let error = test_task_list
                .load_tasks("bad_priority_test.csv")
                .unwrap_err();
            assert_eq!(error.to_string(), "invalid priority");
        }

        #[test]
        fn load_from_csv_sucessful_test() {
            let mut test_task_list = TaskList { tasks: vec![] };
            test_task_list
                .load_tasks("successful_import_test.csv")
                .unwrap();
            let test_task = &test_task_list.tasks[0];
            assert!(test_task.name == "test csv task");
            assert!(test_task.completed == false);
            assert!(test_task.priority == PriEnum::Optional);
        }

        #[test]
        fn load_to_csv_successful_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = TaskList { tasks: vec![] };
            let creation_result = test_task_list.create_task()?;
            assert!(creation_result == "Created new task named Test Task");
            test_task_list.tasks[0].rename_task(&"test csv task".to_string())?;
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
                assert_eq!(record, vec!["test csv task", "Optional", "false"]);
            } else {
                return Ok(());
            }
            return Ok(());
        }

        #[test]
        fn task_creation_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = TaskList { tasks: vec![] };
            let creation_result = test_task_list.create_task()?;
            assert!(creation_result == "Created new task named Test Task");
            let test_task = &test_task_list.tasks[0];
            assert!(test_task.name == "Test Task");
            assert!(test_task.completed == false);
            assert!(test_task.priority == PriEnum::Optional);
            assert!(&test_task_list.tasks[0] == test_task);
            return Ok(());
        }

        #[test]
        fn task_print_fail_test() {
            let test_task_list = TaskList { tasks: vec![] };
            let mut bad_result = Vec::new();
            let error = test_task_list.print_task_list(&mut bad_result).unwrap_err();
            assert_eq!(error.to_string(), "list is empty");
        }

        #[test]
        fn task_print_successful_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = TaskList { tasks: vec![] };
            let mut good_result = Vec::new();
            test_task_list.create_task()?;
            let success = test_task_list.print_task_list(&mut good_result).unwrap();
            assert_eq!(
                &good_result[..],
                "index,name,priority,completed\n0,Test Task,Optional,false\n".as_bytes()
            );
            assert_eq!(success , "End of List");
            return Ok(());
        }

        #[test]
        fn task_removal_fail_test() {
            let mut test_task_list = TaskList { tasks: vec![] };
            let error = test_task_list.remove_task(0).unwrap_err();
            assert_eq!(error.to_string(), "Invalid Index for Deletion");
        }

        #[test]
        fn task_removal_successful_test() {
            let mut test_task_list = TaskList { tasks: vec![] };
            test_task_list.create_task().unwrap();
            let good_result = test_task_list.remove_task(0).unwrap();
            assert_eq!(
                good_result.to_string(),
                "Successfully Removed Task Test Task"
            );
        }
    }

    mod task_tests {
        use super::*;

        #[test]
        fn task_rename_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = TaskList { tasks: vec![] };
            let creation_result = test_task_list.create_task()?;
            assert!(creation_result == "Created new task named Test Task");
            let test_task = &mut test_task_list.tasks[0];
            test_task.rename_task(&"Changed Name".to_string())?;
            assert!(test_task.name == "Changed Name");
            return Ok(());
        }

        #[test]
        fn task_completion_successful_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = TaskList { tasks: vec![] };
            test_task_list.create_task()?;
            let test_task = &mut test_task_list.tasks[0];
            test_task.mark_complete()?;
            assert!(test_task.completed == true);
            return Ok(());
        }

        #[test]
        fn task_completion_fail_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = TaskList { tasks: vec![] };
            test_task_list.create_task()?;
            let test_task = &mut test_task_list.tasks[0];
            test_task.mark_complete()?;
            let failure = test_task.mark_complete().unwrap_err();
            assert_eq!(failure.to_string(), "Task is already completed");
            return Ok(());
        }

        #[test]
        fn task_reprioritize_duplicate_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = TaskList { tasks: vec![] };
            test_task_list.create_task()?;
            let test_task = &mut test_task_list.tasks[0];
            let error = test_task.change_priority("5").unwrap_err();
            assert_eq!(error.to_string(), "duplicate priority");
            return Ok(());
        }

        #[test]
        fn task_reprioritize_failure_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = TaskList { tasks: vec![] };
            test_task_list.create_task()?;
            let test_task = &mut test_task_list.tasks[0];
            let error = test_task.change_priority("6").unwrap_err();
            assert_eq!(error.to_string(), "invalid priority");
            return Ok(());
        }

        #[test]
        fn task_successful_reprioritize_test() -> Result<(), Box<dyn Error>> {
            let mut test_task_list = TaskList { tasks: vec![] };
            test_task_list.create_task()?;
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
