use std::error::Error;
use uuid::Uuid;

use crate::Priority;

pub trait ActionManipulation {
    fn rename(&self, new_action_name: &str) -> Self;
    fn toggle_completion_status(&self) -> Self;
    fn change_priority(&self, new_priority: &str) -> 
    Result<Self, Box<dyn Error>> where Self: Sized;

    fn get_name(&self) -> String;
    fn get_priority(&self) -> Priority;
    fn get_completion_status(&self) -> bool;
    fn get_id(&self) -> Uuid;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fmt::{Display, Formatter, Result as fmtResult};
    use std::io::ErrorKind;


    #[derive(Debug, Clone, PartialEq)]
    pub struct TestStruct {
        pub name: String,
        pub priority: Priority,
        pub completed: bool,
        pub id: Uuid,
    }

    impl Default for TestStruct {
        fn default() -> TestStruct {
            TestStruct {
                name: "Default Struct".to_string(),
                priority: Priority::Low,
                completed: false,
                id: Uuid::nil(),
            }
        }
    }

    impl Display for TestStruct {
        fn fmt(&self, f: &mut Formatter) -> fmtResult {
            write!(f, "{},{},{},{}", self.name, self.priority, self.completed, self.id)
        }
    }

    impl ActionManipulation for TestStruct {
        fn rename(&self, new_name: &str) -> Self {
            TestStruct {
                name: new_name.to_string(),
                priority: self.priority.clone(),
                completed: self.completed.clone(),
                id: self.id.clone(),
            }
        }

        fn toggle_completion_status(&self) -> Self {
            TestStruct {
                name: self.name.to_string(),
                priority: self.priority.clone(),
                completed: !self.completed,
                id: self.id.clone(),
            }
        }

        fn change_priority(&self, new_priority: &str) -> Result<Self, Box<dyn Error>> {
            match new_priority {
                "Low" => Ok(TestStruct {
                    name: self.name.to_string(),
                    priority: Priority::Low,
                    completed: self.completed.clone(),
                    id: self.id.clone(),
                }),
                "Medium" => Ok(TestStruct {
                    name: self.name.to_string(),
                    priority: Priority::Medium,
                    completed: self.completed.clone(),
                    id: self.id.clone(),
                }),
                "High" => Ok(TestStruct {
                    name: self.name.to_string(),
                    priority: Priority::High,
                    completed: self.completed.clone(),
                    id: self.id.clone(),
                }),
                _ => Err(Box::new(std::io::Error::new(ErrorKind::Other, "Invalid Priority"))),
            }
        }


        fn get_id(&self) -> Uuid {
            self.id
        }

        fn get_name(&self) -> String {
            self.name.to_string()
        }
        fn get_priority(&self) -> Priority {
            self.priority
        }
        fn get_completion_status(&self) -> bool {
            self.completed
        }
    }


    #[test]
    fn successful_reprioritization() {
        let test_action = TestStruct::default()
            .change_priority("High")
            .unwrap();
        assert_eq!(test_action.priority, Priority::High);
    }

    #[test]
    fn failed_reprioritization() {
        let test_action_error = TestStruct::default()
            .change_priority("bad_priority")
            .unwrap_err();
        assert_eq!(test_action_error.to_string(), "Invalid Priority".to_string());
    }

    #[test]
    fn successfully_completing_action() {
        let test_action = TestStruct::default().toggle_completion_status();
        assert_eq!(test_action.completed, true);
    }

    #[test]
    fn successfully_reopen_action() {
        let test_action = TestStruct::default()
            .toggle_completion_status()
            .toggle_completion_status();
        assert_eq!(test_action.completed, false);
    }

    #[test]
    fn rename_action() {
        let test_action = TestStruct::default().rename("rename test");
        assert_eq!(test_action.name, "rename test");
    }

    #[test]
    fn get_id() {
        let test_action = TestStruct::default();
        assert_eq!(test_action.get_id(), test_action.id);
    }

    #[test]
    fn get_name() {
        let test_action = TestStruct::default();
        assert_eq!(test_action.get_name(), "Default Struct");
    }

    #[test]
    fn get_priority() {
        let test_action = TestStruct::default();
        assert_eq!(test_action.get_priority(), Priority::Low);
    }

    #[test]
    fn get_completion_status() {
        let test_action = TestStruct::default();
        assert_eq!(test_action.get_completion_status(), false);
    }
}
