use std::error::Error;

pub trait ActionManipulation {
    fn rename(&self, new_action_name: &str) -> Self;
    fn toggle_completion_status(&self) -> Self;
    fn change_priority(&self, new_priority: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn export_fields_as_string(&self) -> String;
    fn create_default() -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as OtherError, ErrorKind};

    #[derive(Debug)]
    struct TestStruct {
        name: String,
        completed: bool,
        priority: String,
    }

    impl Default for TestStruct {
        fn default() -> TestStruct {
            TestStruct {
                name: "Default Struct".to_string(),
                completed: false,
                priority: "low".to_string(),
            }
        }
    }

    impl ActionManipulation for TestStruct {
        fn rename(&self, new_name: &str) -> Self {
            TestStruct {
                name: new_name.to_string(),
                completed: self.completed.clone(),
                priority: self.priority.clone(),
            }
        }

        fn toggle_completion_status(&self) -> Self {
            TestStruct {
                name: self.name.to_string(),
                completed: !self.completed,
                priority: self.priority.to_string(),
            }
        }

        fn change_priority(&self, new_priority: &str) -> Result<Self, Box<dyn Error>> {
            match new_priority {
                "low" => Ok(TestStruct {
                    name: self.name.to_string(),
                    completed: self.completed,
                    priority: new_priority.to_string(),
                }),
                "high" => Ok(TestStruct {
                    name: self.name.to_string(),
                    completed: self.completed,
                    priority: new_priority.to_string(),
                }),
                _ => Err(Box::new(OtherError::new(
                    ErrorKind::Other,
                    "invalid priority",
                ))),
            }
        }

        fn export_fields_as_string(&self) -> String {
            format!(
                "{},{},{}",
                self.name,
                self.completed.to_string(),
                self.priority
            )
        }

        fn create_default() -> Self {
            Self {
                name: "default action".to_string(),
                completed: false,
                priority: "low".to_string(),
            }
        }
    }

    #[test]
    fn successful_default_action_creation() {
        let test_action = TestStruct::create_default();
        assert_eq!(test_action.name, "default action");
        assert_eq!(test_action.completed, false);
        assert_eq!(test_action.priority, "low");
    }

    #[test]
    fn successful_field_export() {
        let test_export = TestStruct::create_default().export_fields_as_string();
        assert_eq!(test_export, "default action,false,low")
    }

    #[test]
    fn successful_reprioritization() {
        let test_action = TestStruct::create_default()
            .change_priority("high")
            .unwrap();
        assert_eq!(test_action.priority, "high");
    }

    #[test]
    fn failed_reprioritization() {
        let test_action_error = TestStruct::create_default()
            .change_priority("bad_priority")
            .unwrap_err();
        assert_eq!(test_action_error.to_string(), "invalid priority".to_string());
    }

    #[test]
    fn successfully_completing_action() {
        let test_action = TestStruct::create_default().toggle_completion_status();
        assert_eq!(test_action.completed, true);
    }

    #[test]
    fn successfully_reopen_action() {
        let test_action = TestStruct::create_default()
            .toggle_completion_status()
            .toggle_completion_status();
        assert_eq!(test_action.completed, false);
    }

    #[test]
    fn rename_action() {
        let test_action = TestStruct::create_default().rename("rename test");
        assert_eq!(test_action.name, "rename test");
    }
}
