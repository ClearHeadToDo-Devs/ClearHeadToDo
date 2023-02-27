use crate::action_interface::*;
use crate::priority::Priority;
use crate::Action;
use uuid::Uuid;

use std::str::FromStr;
use strum::ParseError;

#[derive(Debug)]
pub struct ActionBuilder {
    pub name: String,
    pub completed: bool,
    pub priority: Priority,
    pub id: Uuid,
}

impl ActionEditing for ActionBuilder {
    fn set_name(self: &mut Self, new_name: &str) -> &mut Self {
        self.name = new_name.to_string();

        return self;
    }

    fn set_priority(self: &mut Self, priority: Priority) -> &mut Self {
        self.priority = priority;

        return self;
    }

    fn set_completion_status(self: &mut Self, desired_status: bool) -> &mut Self {
        self.completed = desired_status;

        return self;
    }

    fn set_id(&mut self, id: uuid::Uuid) -> &mut Self {
        self.id = id;

        return self;
    }
}

impl ActionViewing for ActionBuilder {
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

impl ActionBuilder {
    pub fn build(self: &Self) -> Action {
        return Action::default()
            .set_name(&self.name)
            .set_priority(self.priority)
            .set_completion_status(self.completed)
            .set_id(self.id)
            .to_owned();
    }
}

impl Default for ActionBuilder {
    fn default() -> Self {
        ActionBuilder {
            name: "Default Action".to_string(),
            completed: false,
            priority: Priority::Optional,
            id: Uuid::new_v4(),
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

        test_builder.set_priority(Priority::Optional);

        assert!(test_builder.priority == Priority::Optional);
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
        let test_builder_1 = ActionBuilder::default();
        let test_builder_2 = ActionBuilder::default();

        let action_1 = test_builder_1.build();
        let action_2 = test_builder_2.build();

        assert!(action_1.id != action_2.id)
    }

    #[test]
    fn create_custom_action() {
        let mut test_builder = ActionBuilder::default();

        let custom_action = test_builder
            .set_completion_status(true)
            .set_name("Custom Action")
            .set_priority(Priority::Critical)
            .build();

        assert!(
            custom_action.get_name() == "Custom Action"
                && custom_action.get_priority() == &Priority::Critical
                && custom_action.get_completion_status() == true
                && custom_action.get_id().is_nil() == false
        )
    }
}
