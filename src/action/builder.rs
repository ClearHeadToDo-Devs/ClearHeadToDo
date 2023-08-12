use crate::Action;
use crate::action::ActionEditing;
use crate::action::ActionViewing;
use crate::action::Priority;
use uuid::Uuid;

#[derive(Debug)]
pub struct ActionBuilder {
    pub name: String,
    pub completed: bool,
    pub priority: Priority,
    pub id: Uuid,
}

impl ActionEditing for ActionBuilder {
    fn set_name(&mut self, new_name: &str) -> &mut Self {
        self.name = new_name.to_string();

        self
    }

    fn set_priority(&mut self, priority: Priority) -> &mut Self {
        self.priority = priority;

        self
    }

    fn set_completion_status(&mut self, desired_status: bool) -> &mut Self {
        self.completed = desired_status;

        self
    }

    fn set_id(&mut self, id: uuid::Uuid) -> &mut Self {
        self.id = id;

        self
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

    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl ActionBuilder {
    pub fn build(&self) -> Action {
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
mod tests {
    use super::*;

    #[test]
    fn update_builder_name() {
        let mut test_builder = ActionBuilder::default();

        test_builder.set_name("New Name");

        assert!(test_builder.get_name() == "New Name")
    }

    #[test]
    fn update_builder_priority() {
        let mut test_builder = ActionBuilder::default();

        test_builder.set_priority(Priority::Optional);

        assert!(*test_builder.get_priority() == Priority::Optional);
    }

    #[test]
    fn update_builder_completion_status() {
        let mut test_builder = ActionBuilder::default();

        test_builder.set_completion_status(true);

        assert!(test_builder.get_completion_status());
    }

    #[test]
    fn build_default_action() {
        let test_builder = ActionBuilder::default();

        let test_action = test_builder.build();

        assert!(
        test_action.get_name() == "Default Action");
        assert!(test_action.get_priority() == &Priority::Optional);
        assert!(!test_action.get_completion_status());
        assert!(!test_action.id.is_nil());
    }

    #[test]
    fn create_multiple_actions_from_builder() {
        let mut test_builder = ActionBuilder::default();

        let action_1 = test_builder.build();
        let action_2 = test_builder.set_id(Uuid::new_v4()).build();

        assert!(action_1.id != action_2.id)
    }
}
