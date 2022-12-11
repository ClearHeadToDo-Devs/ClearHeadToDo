use uuid::Uuid;
use std::error::Error;

use crate::action::Action;
use crate::ActionManipulation;
use std::str::FromStr;
use crate::Priority;

pub trait ActionBuilding {
    fn get_name(&self) -> String;
    fn get_priority(&self) -> String;
    fn get_completion_status(&self) -> bool;
    fn get_id(&self) -> Uuid;

    fn set_name(&mut self, name: &str);
    fn set_priority(&mut self, priority: &str) -> Result<(), Box<dyn Error>>;
    fn toggle_completed(&mut self);

    fn build(&self) -> Action;
}

#[derive(PartialEq, Debug)]
pub struct ActionBuilder {
    name: String,
    priority: Priority,
    completed: bool,
    id: Uuid,
}

impl Default for ActionBuilder {
    fn default() -> ActionBuilder {
        ActionBuilder {
            id: Uuid::new_v4(),
            name: "Default Action".to_string(),
            completed: false,
            priority: Default::default(),
        }
    }
}

impl ActionBuilding for ActionBuilder {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_priority(&self) -> String {
        self.priority.to_string()
    }
    fn get_completion_status(&self) -> bool {
        self.completed.clone()
    }
    fn get_id(&self) -> Uuid {
        self.id.clone()
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
    fn set_priority(&mut self, priority: &str) -> Result<(), Box<dyn Error>> {
        self.priority = Priority::from_str(priority)?;
        Ok(())
    }
    fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }

    fn build(&self) -> Action {
        let default_action = Action::default();
        
        let new_name = default_action.rename(&self.get_name());
        let new_priority = new_name.change_priority(&self.get_priority()).unwrap();
        let new_completion_status_action = new_priority;
        if self.completed == true {
            new_completion_status_action.toggle_completion_status();
        }

        return new_completion_status_action
    }
}

mod tests {
    use super::*;

    #[test]
    fn create_default_builder() {
        let test_builder = ActionBuilder::default();

        assert_eq!(test_builder.name, "Default Action");
        assert_eq!(test_builder.priority, Priority::Optional);
        assert_eq!(test_builder.completed, false);
        assert_eq!(test_builder.id.is_nil(), false);
    }

    #[test]
    fn get_name_from_builder() {
        let test_builder = ActionBuilder::default();

        assert_eq!(test_builder.get_name(), "Default Action");
    }

    #[test]
    fn get_priority_from_builder() {
        let test_builder = ActionBuilder::default();
        
        let builder_priority = test_builder.get_priority();
       
        assert_eq!(builder_priority, Priority::Optional.to_string());
    }

    #[test]
    fn get_completion_status_from_builder() {
        let test_builder = ActionBuilder::default();

        assert_eq!(test_builder.get_completion_status(), false);
    }

    #[test]
    fn get_id_from_builder() {
        let test_builder = ActionBuilder::default();

        assert_eq!(test_builder.get_id().is_nil(), false);
    }

    #[test]
    fn set_builder_name() {
        let mut test_builder = ActionBuilder::default();

        test_builder.set_name("Test Name");

        assert_eq!(test_builder.name, "Test Name");
    }

    #[test]
    fn set_builder_priority() {
        let mut test_builder = ActionBuilder::default();

        test_builder.set_priority("High").unwrap();

        assert_eq!(test_builder.priority, Priority::High);
    }

    #[test]
    fn failed_set_builder_priority() {
        let mut test_builder = ActionBuilder::default();

        let priority_error = test_builder.set_priority("Not a priority").unwrap_err();

        assert_eq!(
            priority_error.to_string(),
            "Not a priority is an Invalid Priority Option"
        );
    }

    #[test]
    fn set_builder_completion_status() {
        let mut test_builder = ActionBuilder::default();

        test_builder.toggle_completed();

        assert_eq!(test_builder.completed, true);
    }

    #[test]
    fn build_default_task() {
        let test_builder = ActionBuilder::default();

        let test_action = test_builder.build();

        assert_eq!(test_action.get_name(), "Default Action".to_string());
        assert_eq!(test_action.get_priority(), Priority::Optional.to_string());
        assert_eq!(test_action.get_completion_status(), false);
        assert_eq!(test_action.get_id().is_nil(), false);
    }
}
