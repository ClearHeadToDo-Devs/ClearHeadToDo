use std::error::Error;
use uuid::Uuid;

pub trait ActionManipulation {
    fn rename(&self, new_action_name: &str) -> Self;
    fn toggle_completion_status(&self) -> Self;
    fn change_priority(&self, new_priority: &str) -> 
    Result<Self, Box<dyn Error>> where Self: Sized;
    fn get_id(&self) -> Uuid;
    fn get_name(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as OtherError, ErrorKind};

    #[derive(Debug)]
    struct TestStruct {
        name: String,
        priority: String,
        completed: bool,
        id: Uuid,
    }

    impl Default for TestStruct {
        fn default() -> TestStruct {
            TestStruct {
                name: "Default Struct".to_string(),
                priority: "low".to_string(),
                completed: false,
                id: Uuid::nil(),
            }
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
                priority: self.priority.to_string(),
                completed: !self.completed,
                id: self.id.clone(),
            }
        }

        fn change_priority(&self, new_priority: &str) -> Result<Self, Box<dyn Error>> {
            match new_priority {
                "low" => Ok(TestStruct {
                    name: self.name.to_string(),
                    priority: new_priority.to_string(),
                    completed: self.completed,
                    id: self.id.clone(),
                }),
                "high" => Ok(TestStruct {
                    name: self.name.to_string(),
                    priority: new_priority.to_string(),
                    completed: self.completed,
                    id: self.id.clone(),
                }),
                _ => Err(Box::new(OtherError::new(
                    ErrorKind::Other,
                    "invalid priority",
                ))),
            }
        }


        fn get_id(&self) -> Uuid {
            self.id
        }

        fn get_name(&self) -> String {
            self.name.to_string()
        }
    }

    #[test]
    fn successful_reprioritization() {
        let test_action = TestStruct::default()
            .change_priority("high")
            .unwrap();
        assert_eq!(test_action.priority, "high");
    }

    #[test]
    fn failed_reprioritization() {
        let test_action_error = TestStruct::default()
            .change_priority("bad_priority")
            .unwrap_err();
        assert_eq!(test_action_error.to_string(), "invalid priority".to_string());
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
}
