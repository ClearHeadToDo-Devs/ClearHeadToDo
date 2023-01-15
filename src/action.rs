use std::str::FromStr;
use strum::*;
use uuid::Uuid;

use crate::priority::Priority;
use crate::action_interface::*;

use crate::ActionBuilder;


pub struct Action {
    pub name: String,
    pub completed: bool,
    pub priority: Priority,
    pub id: Uuid,
}

impl ActionEditing for Action {
    fn set_name(self: &mut Self, new_name: &str) -> &mut Self {
        self.name = new_name.to_string();

        return self;
    }

    fn set_priority(self: &mut Self, priority_str: &str) -> Result<&mut Self, ParseError> {
        self.priority = Priority::from_str(priority_str)?;

        Ok(self)
    }

    fn set_completion_status(self: &mut Self, desired_status: bool) -> &mut Self {
        self.completed = desired_status;

        return self;
    }
}

impl ActionViewing for Action {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_priority(&self) -> &Priority {
        &self.priority
    }

    fn get_completion_status(&self) -> bool {
        self.completed
    }
}

impl Default for Action {
    fn default() -> Self {
        Action {
            name: "Default Action".to_string(),
            completed: false,
            priority: Priority::Optional,
            id: Uuid::new_v4(),
        }
    }
}

#[cfg(test)]
mod object {
    use super::*;

    #[test]
    fn create_default_action() {
        let test_action = Action::default();

        assert!(
            test_action.name == "Default Action"
                && test_action.priority == Priority::Optional
                && test_action.completed == false
                && test_action.id.is_nil() == false
        );
    }
    #[test]
    fn update_action_name() {
        let mut test_action = ActionBuilder::default().build();

        test_action.set_name("New Name");

        assert!(test_action.name == "New Name")
    }

    #[test]
    fn update_action_priority() {
        let mut test_action = ActionBuilder::default().build();

        test_action.set_priority("Critical").unwrap();

        assert!(test_action.priority == Priority::Critical)
    }

    #[test]
    fn update_action_completion_status() {
        let mut test_action = ActionBuilder::default().build();

        test_action.set_completion_status(true);

        assert!(test_action.completed == true);
    }
}
