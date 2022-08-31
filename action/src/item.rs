use crate::priority::*;

use crate::action_manipulation::ActionManipulation;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct Action {
    pub name: String,
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
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{},{},{},{}", self.name, self.priority, self.completed, self.id)
    }
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
        let new_pri: Priority = Priority::parse_priority(new_priority)?;
        return Ok(Action {
            priority: new_pri.clone(),
            ..self.to_owned()
        });
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::Error;
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
    fn action_creation_unique_id() {
        let first_test_action = Action::default();
        let second_test_action = Action::default();

        assert!(first_test_action.id != second_test_action.id);
    }

    #[test]
    fn rename() {
        let test_action = Action::default();
        let renamed_action = &test_action.rename(&"Changed Name".to_string());

        assert!(renamed_action.name == "Changed Name");
    }

    #[test]
    fn completion() -> Result<(), Box<dyn Error>> {
        let test_action = Action::default();
        let test_successful_completion_action = &test_action.toggle_completion_status();

        assert!(test_successful_completion_action.completed == true);
        return Ok(());
    }

    #[test]
    fn reopen() -> () {
        let test_action = Action::default();
        let test_first_completion_action = &test_action.toggle_completion_status();
        let reopened_action = &test_first_completion_action.toggle_completion_status();
        assert_eq!(reopened_action.completed, false);
    }

    #[test]
    fn failing_reprioritize() -> Result<(), Box<dyn Error>> {
        let test_action = Action::default();
        let error = &test_action.change_priority("6").unwrap_err();
        assert_eq!(error.to_string(), "6 is an Invalid Priority Option");
        return Ok(());
    }

    #[test]
    fn successful_reprioritize() -> Result<(), Box<dyn Error>> {
        let priority_5_test_action = Action::default();

        let priority_4_test_action = &priority_5_test_action.change_priority("4")?;
        assert!(priority_4_test_action.priority == Priority::Low);

        let priority_3_test_action = &priority_4_test_action.change_priority("3")?;
        assert!(priority_3_test_action.priority == Priority::Medium);

        let priority_2_test_action = &priority_3_test_action.change_priority("2")?;
        assert!(priority_2_test_action.priority == Priority::High);

        let priority_1_test_action = &priority_2_test_action.change_priority("1")?;
        assert!(priority_1_test_action.priority == Priority::Critical);

        return Ok(());
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
