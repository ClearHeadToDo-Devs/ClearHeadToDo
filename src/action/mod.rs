use uuid::Uuid;

use serde::{Deserialize, Serialize};

pub mod builder;
pub mod interface;
use interface::*;
pub mod priority;
use priority::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Action {
    pub name: String,
    pub completed: bool,
    pub priority: Priority,
    pub id: Uuid,
}

impl ActionEditing for Action {
    fn set_name(&mut self, new_name: &str) -> &mut Self {
        self.name = new_name.to_string();

        self
    }

    fn set_priority(&mut self, new_priority: Priority) -> &mut Self {
        self.priority = new_priority;

        self
    }

    fn set_completion_status(&mut self, desired_status: bool) -> &mut Self {
        self.completed = desired_status;

        self
    }

    fn set_id(&mut self, id: Uuid) -> &mut Self {
        self.id = id;

        self
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

    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl Action {}

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
mod test {
    use rstest::*;

    use super::*;
    use builder::ActionBuilder;


    #[test]
    fn view_id() {
        let mut action = Action::default();

        let updated_action = action.set_id(Uuid::new_v4());

        let id = updated_action.get_id();

        assert!(!id.is_nil());
    }

    #[rstest]
    fn create_default_action(default_action: Action) {
        assert!(
            default_action.get_name() == "Default Action"
                && *default_action.get_priority() == Priority::Optional
                && !default_action.get_completion_status()
                && !default_action.get_id().is_nil()
        );
    }
    #[rstest]
    fn update_action_name(mut default_action: Action) {
        let updated_action = default_action.set_name("New Name");

        assert!(updated_action.get_name() == "New Name")
    }

    #[rstest]
    fn update_action_priority(mut default_action: Action) {
        let updated_action = default_action.set_priority(Priority::Critical);

        assert!(*updated_action.get_priority() == Priority::Critical)
    }

    #[rstest]
    fn update_action_completion_status(mut default_action: Action) {
        let updated_action = default_action.set_completion_status(true);

        assert!(updated_action.get_completion_status());
    }

    #[fixture]
    fn default_action()->Action {
        Action::default()
    }
}

