use std::error::Error;

pub trait TaskManipulation {
    fn rename(&self, new_task_name: &str) -> Self;
    fn toggle_completion_status(&self) -> Self;
    fn change_priority(&self, new_priority: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn export_fields_as_string(&self) -> String;
    fn create_default_task() -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

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

    impl TaskManipulation for TestStruct {
        fn rename(&self, new_task_name: &str) -> Self {
            TestStruct {
                name: new_task_name.to_string(),
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
            Ok(TestStruct {
                name: self.name.to_string(),
                completed: self.completed,
                priority: new_priority.to_string(),
            })
        }

        fn export_fields_as_string(&self) -> String {
            format!(
                "{},{},{}",
                self.name,
                self.completed.to_string(),
                self.priority
            )
        }

        fn create_default_task() -> Self {
            Self {
                name: "default task".to_string(),
                completed: false,
                priority: "low".to_string(),
            }
        }
    }

    #[test]
    fn successful_default_task_creation() {
        let test_task = TestStruct::create_default_task();
        assert_eq!(test_task.name, "default task");
        assert_eq!(test_task.completed, false);
        assert_eq!(test_task.priority, "low");
    }
    #[test]
    fn successful_field_export() {
        let test_export = TestStruct::create_default_task().export_fields_as_string();
        assert_eq!(test_export, "default task,false,low")
    }
}
