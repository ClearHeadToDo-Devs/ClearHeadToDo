use crate::priority::Priority;
use crate::action_interface::*;
use crate::Action;

use strum::ParseError;
use uuid::Uuid;
use std::str::FromStr;


#[derive(Debug)]
pub struct ActionBuilder {
    name: String,
    completed: bool,
    priority: Priority,
}
impl ActionEditing for ActionBuilder {
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

impl ActionBuilder {
    pub fn build(self: &Self) -> Action {
        Action {
            name: self.name.to_string(),
            completed: self.completed,
            priority: self.priority,
            id: Uuid::new_v4(),
        }
    }
}

impl Default for ActionBuilder {
    fn default() -> Self {
        ActionBuilder {
            name: "Default Action".to_string(),
            completed: false,
            priority: Priority::Optional,
        }
    }
}

#[cfg(test)]
mod builder {
    use super::*;

    #[test]
    fn create_default_builder() {
        let test_builder = ActionBuilder::default();

        assert!(
            test_builder.name == "Default Action"
                && test_builder.completed == false
                && test_builder.priority == Priority::Optional
        )
    }

    #[test]
    fn update_builder_name() {
        let mut test_builder = ActionBuilder::default();

        test_builder.set_name("New Name");

        assert!(test_builder.name == "New Name")
    }

    #[test]
    fn update_builder_priority() {
        let mut test_builder = ActionBuilder::default();

        test_builder.set_priority("Optional").unwrap();

        assert!(test_builder.priority == Priority::Optional);
    }

    #[test]
    fn failed_update_builder_priority() {
        let mut test_builder = ActionBuilder::default();

        let failure_message = test_builder.set_priority("Bad Priority").unwrap_err();

        assert!(failure_message == ParseError::VariantNotFound);
    }

    #[test]
    fn update_builder_completion_status() {
        let mut test_builder = ActionBuilder::default();

        test_builder.set_completion_status(true);

        assert!(test_builder.completed == true);
    }

    #[test]
    fn build_default_action() {
        let test_builder = ActionBuilder::default();

        let test_action = test_builder.build();

        assert!(
            test_action.get_name() == "Default Action"
                && test_action.get_priority() == &Priority::Optional
                && test_action.get_completion_status() == false
                && test_action.id.is_nil() == false
        )
    }

    #[test]
    fn create_multiple_actions_from_builder() {
        let test_builder = ActionBuilder::default();

        let action_1 = test_builder.build();
        let action_2 = test_builder.build();

        assert!(action_1.id != action_2.id)
    }

    #[test]
    fn create_custom_action() {
        let mut test_builder = ActionBuilder::default();

        let custom_action = test_builder
            .set_completion_status(true)
            .set_name("Custom Action")
            .set_priority("Critical")
            .unwrap()
            .build();

        assert!(
            custom_action.name == "Custom Action"
                && custom_action.priority == Priority::Critical
                && custom_action.completed == true
                && custom_action.id.is_nil() == false
        )
    }
}
