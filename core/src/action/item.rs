pub use crate::priority::*;

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize, Tabled)]
pub struct Action {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Priority")]
    priority: Priority,

    #[tabled(rename = "Completed")]
    completed: bool,

    #[tabled(skip)]
    id: Uuid,
}

impl Default for Action {
    fn default() -> Action {
        Action {
            id: Uuid::new_v4(),
            name: "Default Action".to_string(),
            completed: false,
            priority: Default::default(),
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.name, self.priority, self.completed, self.id
        )
    }
}

pub trait ActionManipulation {
    fn rename(&self, new_action_name: &str) -> Action;
    fn toggle_completion_status(&self) -> Action;
    fn change_priority(&self, new_priority: &str) -> Result<Action, Box<dyn Error>>;

    fn get_id(&self) -> Uuid;
    fn get_name(&self) -> String;
    fn get_priority(&self) -> String;
    fn get_completion_status(&self) -> bool;
}

impl ActionManipulation for Action {
    fn rename(&self, new_action_name: &str) -> Action {
        return Action {
            name: new_action_name.to_owned(),
            ..self.to_owned()
        };
    }

    fn toggle_completion_status(&self) -> Action {
        Action {
            completed: !self.completed,
            ..self.to_owned()
        }
    }

    fn change_priority(&self, new_priority: &str) -> Result<Action, Box<dyn Error>> {
        return Ok(Action {
            priority: Priority::from_str(new_priority)?,
            ..self.to_owned()
        });
    }

    fn get_id(&self) -> Uuid {
        self.id.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_priority(&self) -> String {
        self.priority.to_string()
    }

    fn get_completion_status(&self) -> bool {
        self.completed.clone()
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct ActionBuilder{

}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, assert_ser_tokens, Configure, Token};
    use uuid::Uuid;

    pub fn create_nil_action() -> Action {
        Action {
            id: Uuid::nil(),
            ..Default::default()
        }
    }

    #[test]
    fn default_action_creation() {
        let test_action = create_nil_action();
        assert!(test_action.get_name() == "Default Action".to_string());
        assert!(test_action.get_priority() == "Optional");
        assert!(test_action.get_completion_status() == false);
        assert!(test_action.get_id() == Uuid::nil());
    }

    #[test]
    fn print_action_content() {
        let test_action = create_nil_action();

        let test_action_string = test_action.to_string();

        assert_eq!(
            test_action_string,
            "Default Action,Optional,false,00000000-0000-0000-0000-000000000000",
        );
    }

    #[test]
    fn get_name() {
        let test_action = Action::default();

        assert_eq!(test_action.get_name(), "Default Action".to_string());
    }

    #[test]
    fn get_priority() {
        let test_action = Action::default();

        assert_eq!(test_action.get_priority(), Priority::Optional.to_string());
    }

    #[test]
    fn get_completion_status() {
        let test_action = Action::default();

        assert_eq!(test_action.get_completion_status(), false);
    }

    #[test]
    fn get_id() {
        let test_action = create_nil_action();

        assert_eq!(test_action.get_id(), Uuid::nil());
    }

    #[test]
    fn rename_action() {
        let test_action = Action::default();

        let renamed_action = test_action.rename("New Action Name");

        assert_eq!(renamed_action.get_name(), "New Action Name".to_string());
    }

    #[test]
    fn toggle_completion_status() {
        let test_action = Action::default();

        let completed_action = test_action.toggle_completion_status();

        assert_eq!(completed_action.get_completion_status(), true);
    }

    #[test]
    fn reopen_action() {
        let test_action = Action::default();

        let reopened_action = test_action
            .toggle_completion_status()
            .toggle_completion_status();

        assert_eq!(reopened_action.get_completion_status(), false);
    }

    #[test]
    fn reprioritize_action() {
        let test_action = Action::default();

        let reprioritized_action = test_action.change_priority("High").unwrap();

        assert_eq!(
            reprioritized_action.get_priority(),
            Priority::High.to_string()
        );
    }

    #[test]
    fn failed_reprioritize_action() {
        let test_action = Action::default();

        let reprioritization_error = test_action.change_priority("Not a priority").unwrap_err();

        assert_eq!(
            reprioritization_error.to_string(),
            "Not a priority is an Invalid Priority Option"
        );
    }

    #[test]
    fn successfully_serialize_action() {
        let test_action = Action {
            id: Uuid::nil(),
            ..Default::default()
        };

        assert_ser_tokens(
            &test_action.readable(),
            &[
                Token::Struct {
                    name: "Action",
                    len: 4,
                },
                Token::Str("name"),
                Token::Str("Default Action"),
                Token::Str("priority"),
                Token::UnitVariant {
                    name: "Priority",
                    variant: "Optional",
                },
                Token::Str("completed"),
                Token::Bool(false),
                Token::Str("id"),
                Token::Str("00000000-0000-0000-0000-000000000000"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn successfully_deserializing_action() {
        let test_action = Action {
            id: Uuid::nil(),
            ..Default::default()
        };
        assert_de_tokens(
            &test_action.readable(),
            &[
                Token::Struct {
                    name: "Action",
                    len: 5,
                },
                Token::Str("name"),
                Token::Str("Default Action"),
                Token::Str("priority"),
                Token::UnitVariant {
                    name: "Priority",
                    variant: "Optional",
                },
                Token::Str("completed"),
                Token::Bool(false),
                Token::Str("id"),
                Token::Str("00000000-0000-0000-0000-000000000000"),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    fn create_default_builder(){
        let test_builder = ActionBuilder::default();

        assert_eq!(test_builder, ActionBuilder {});
    }
}
