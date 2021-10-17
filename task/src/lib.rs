use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Serialize as AltSerialize;
use std::error::Error;
use std::fmt;
use std::io::{Error as OtherError, ErrorKind};
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

pub trait TaskManipulation {
    fn rename(&self, new_task_name: &str) -> Self;
    fn toggle_completion_status(&self) -> Self;
    fn change_priority(&self, new_priority: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn export_fields_as_string(&self) -> String;
}

impl TaskManipulation for Task {
    fn rename(&self, new_task_name: &str) -> Task {
        return Task {
            name: new_task_name.to_owned(),
            id: self.id,
            priority: self.priority,
            completed: self.completed,
        };
    }

    fn toggle_completion_status(&self) -> Task {
        Task {
            id: self.id.clone(),
            name: self.name.clone(),
            priority: self.priority.clone(),
            completed: !self.completed,
        }
    }

    fn change_priority(&self, new_priority: &str) -> Result<Task, Box<dyn Error>> {
        let new_pri: PriEnum = parse_priority(new_priority)?;
        return Ok(Task {
            name: self.name.clone(),
            priority: new_pri.clone(),
            id: self.id.clone(),
            completed: self.completed.clone(),
        });
    }

    fn export_fields_as_string(&self) -> String {
        format!(
            "{name},{priority},{completed},{ID}\n",
            name = self.name,
            priority = self.priority.to_string(),
            completed = self.completed,
            ID = self.id
        )
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
    fn print_task_content_test() {
        let test_task = create_nil_task();
        let test_task_string = test_task.export_fields_as_string();
        assert_eq!(
            test_task_string,
            "Default Task,Optional,false,00000000-0000-0000-0000-000000000000\n",
        );
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
        let renamed_task = &test_task.rename(&"Changed Name".to_string());

        assert!(renamed_task.name == "Changed Name");
    }

    #[test]
    fn completion_test() -> Result<(), Box<dyn Error>> {
        let test_task = create_default_task();
        let test_successful_completion_task = &test_task.toggle_completion_status();

        assert!(test_successful_completion_task.completed == true);
        return Ok(());
    }

    #[test]
    fn reopen_test() -> () {
        let test_task = create_default_task();
        let test_first_completion_task = &test_task.toggle_completion_status();
        let reopened_task = &test_first_completion_task.toggle_completion_status();
        assert_eq!(reopened_task.completed, false);
    }

    #[test]
    fn failing_reprioritize_test() -> Result<(), Box<dyn Error>> {
        let test_task = create_default_task();
        let error = &test_task.change_priority("6").unwrap_err();
        assert_eq!(error.to_string(), "invalid priority");
        return Ok(());
    }

    #[test]
    fn successful_reprioritize_test() -> Result<(), Box<dyn Error>> {
        let priority_5_test_task = create_default_task();

        let priority_4_test_task = &priority_5_test_task.change_priority("4")?;
        assert!(priority_4_test_task.priority == PriEnum::Low);

        let priority_3_test_task = &priority_4_test_task.change_priority("3")?;
        assert!(priority_3_test_task.priority == PriEnum::Medium);

        let priority_2_test_task = &priority_3_test_task.change_priority("2")?;
        assert!(priority_2_test_task.priority == PriEnum::High);

        let priority_1_test_task = &priority_2_test_task.change_priority("1")?;
        assert!(priority_1_test_task.priority == PriEnum::Critical);

        return Ok(());
    }
}
