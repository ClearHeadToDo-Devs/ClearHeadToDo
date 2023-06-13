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
    fn set_name(self: &mut Self, new_name: &str) -> &mut Self {
        self.name = new_name.to_string();

        return self;
    }

    fn set_priority(self: &mut Self, new_priority: Priority) -> &mut Self {
        self.priority = new_priority;

        return self;
    }

    fn set_completion_status(self: &mut Self, desired_status: bool) -> &mut Self {
        self.completed = desired_status;

        return self;
    }

    fn set_id(&mut self, id: Uuid) -> &mut Self {
        self.id = id;

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

    fn get_id(self: &Self) -> Uuid {
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
    use serde_test::assert_tokens;

    use super::*;
    use builder::ActionBuilder;
    use serde_test::*;

    #[test]
    fn view_name() {
        let action = Action::default();

        let name = action.get_name();

        assert!(name == "Default Action");
    }

    #[test]
    fn view_priority() {
        let action = Action::default();

        let priority = action.get_priority();

        assert!(priority == &Priority::Optional);
    }

    #[test]
    fn view_completion_status() {
        let action = Action::default();

        let completion_status = action.get_completion_status();

        assert!(completion_status == false);
    }

    #[test]
    fn view_id() {
        let action = Action::default();

        let id = action.get_id();

        assert!(id != Uuid::nil());
    }

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

        test_action.set_priority(Priority::Critical);

        assert!(test_action.priority == Priority::Critical)
    }

    #[test]
    fn update_action_completion_status() {
        let mut test_action = ActionBuilder::default().build();

        test_action.set_completion_status(true);

        assert!(test_action.completed == true);
    }

    #[test]
    fn serialize_deserialize() {
        let nil_action = Action {
            name: "Default Action".to_string(),
            completed: false,
            priority: Priority::Optional,
            id: Uuid::nil(),
        };

        assert_tokens(
            &nil_action.readable(),
            &[
                Token::Struct {
                    name: "Action",
                    len: 4,
                },
                Token::Str("name"),
                Token::Str("Default Action"),
                Token::Str("completed"),
                Token::Bool(false),
                Token::Str("priority"),
                Token::UnitVariant {
                    name: "Priority",
                    variant: "Optional",
                },
                Token::Str("id"),
                Token::Str("00000000-0000-0000-0000-000000000000"),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    fn list_serialize_and_deserialize() {
        let action_list = vec![Action {
            name: "Default Action".to_string(),
            completed: false,
            priority: Priority::Optional,
            id: Uuid::nil(),
        }];

        assert_tokens(
            &action_list.readable(),
            &[
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Action",
                    len: 4,
                },
                Token::Str("name"),
                Token::Str("Default Action"),
                Token::Str("completed"),
                Token::Bool(false),
                Token::Str("priority"),
                Token::UnitVariant {
                    name: "Priority",
                    variant: "Optional",
                },
                Token::Str("id"),
                Token::Str("00000000-0000-0000-0000-000000000000"),
                Token::StructEnd,
                Token::SeqEnd,
            ],
        )
    }
}