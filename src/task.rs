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

#[repr(u8)]
#[derive(AltSerialize, Copy, Clone, PartialEq, Debug)]
pub enum PriEnum {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Optional = 5,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub completed: bool,
    pub priority: PriEnum,
}

pub fn create_default_task() -> Task {
    Task {
        ..Default::default()
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

impl fmt::Display for PriEnum {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let printable: &str = match *self {
            PriEnum::Critical => "Critical",
            PriEnum::High => "High",
            PriEnum::Medium => "Medium",
            PriEnum::Low => "Low",
            PriEnum::Optional => "Optional",
        };
        write!(formatter, "{}", printable)
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

#[cfg(test)]
mod test {
    use super::*;

    pub fn create_nil_task() -> Task {
        Task {
            id: Uuid::nil(),
            ..Default::default()
        }
    }

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
