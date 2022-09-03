pub use crate::priority::*;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use tabled::Tabled;


#[derive(PartialEq, Debug, Clone, Deserialize, Serialize, Tabled)]
pub struct Action {
    name: String,
    pub priority: Priority,
    pub completed: bool,
    pub id: Uuid,
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
        write!(f, "{},{},{},{}", self.name, self.priority, self.completed, self.id)
    }
}

impl Action {
    pub fn rename(&self, new_action_name: &str) -> Action {
        return Action {
            name: new_action_name.to_owned(),
            ..self.to_owned()
        };
    }
    pub fn toggle_completion_status(&self) -> Action {
        Action {
            completed: !self.completed,
            ..self.to_owned()
        }
    }
    pub fn change_priority(&self, new_priority: &str) -> Result<Action, Box<dyn Error>> {
        return Ok(Action {
            priority: Priority::from_str(new_priority)?,
            ..self.to_owned()
        });
    }

    pub fn get_id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_priority(&self) -> Priority {
        self.priority.clone()
    }

    pub fn get_completion_status(&self) -> bool {
        self.completed.clone()
    }
}

#[cfg(test)]
pub mod tests{
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
        assert!(test_action.name == "Default Action".to_string());
        assert!(test_action.priority == Priority::Optional);
        assert!(test_action.completed == false);
        assert!(test_action.id.to_string() == "00000000-0000-0000-0000-000000000000".to_string());
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
        let test_action =  Action {
            id: Uuid::nil(),
            ..Default::default()
        };
    assert_de_tokens(&test_action.readable(), &[
        Token::Struct {name: "Action", len:5},
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
    ])
    }
}
