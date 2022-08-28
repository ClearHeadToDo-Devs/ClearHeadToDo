use crate::priority::*;

use crate::action_manipulation::ActionManipulation;

use std::error::Error;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct Action {
    pub id: Uuid,
    pub name: String,
    pub completed: bool,
    pub priority: PriEnum,
}

impl Default for Action {
    fn default() -> Action {
        Action {
            id: Uuid::new_v4(),
            name: "Default Task".to_string(),
            completed: false,
            priority: Default::default(),
        }
    }
}

impl ActionManipulation for Action {
    fn rename(&self, new_task_name: &str) -> Action {
        return Action {
            name: new_task_name.to_owned(),
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
        let new_pri: PriEnum = PriEnum::parse_priority(new_priority)?;
        return Ok(Action {
            priority: new_pri.clone(),
            ..self.to_owned()
        });
    }

    fn export_fields_as_string(&self) -> String {
        format!(
            "{name},{priority},{completed},{ID}\n",
            name = self.name,
            priority = self.priority.to_string(),
            completed = self.completed,
            ID = self.id
        )
    }

    fn create_default() -> Action {
        Action {
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::Error;
    use serde_test::{assert_de_tokens, assert_ser_tokens, Configure, Token};
    use uuid::Uuid;

    pub fn create_nil_task() -> Action {
        Action {
            id: Uuid::nil(),
            ..Default::default()
        }
    }

    #[test]
    fn default_task_creation() {
        let test_task = create_nil_task();
        assert!(test_task.name == "Default Task".to_string());
        assert!(test_task.priority == PriEnum::Optional);
        assert!(test_task.completed == false);
        assert!(test_task.id.to_string() == "00000000-0000-0000-0000-000000000000".to_string());
    }

    #[test]
    fn print_task_content() {
        let test_task = create_nil_task();
        let test_task_string = test_task.export_fields_as_string();
        assert_eq!(
            test_task_string,
            "Default Task,Optional,false,00000000-0000-0000-0000-000000000000\n",
        );
    }

    #[test]
    fn task_creation_unique_id() {
        let first_test_task = Action::create_default();
        let second_test_task = Action::create_default();

        assert!(first_test_task.id != second_test_task.id);
    }

    #[test]
    fn rename() {
        let test_task = Action::create_default();
        let renamed_task = &test_task.rename(&"Changed Name".to_string());

        assert!(renamed_task.name == "Changed Name");
    }

    #[test]
    fn completion() -> Result<(), Box<dyn Error>> {
        let test_task = Action::create_default();
        let test_successful_completion_task = &test_task.toggle_completion_status();

        assert!(test_successful_completion_task.completed == true);
        return Ok(());
    }

    #[test]
    fn reopen() -> () {
        let test_task = Action::create_default();
        let test_first_completion_task = &test_task.toggle_completion_status();
        let reopened_task = &test_first_completion_task.toggle_completion_status();
        assert_eq!(reopened_task.completed, false);
    }

    #[test]
    fn failing_reprioritize() -> Result<(), Box<dyn Error>> {
        let test_task = Action::create_default();
        let error = &test_task.change_priority("6").unwrap_err();
        assert_eq!(error.to_string(), "invalid priority");
        return Ok(());
    }

    #[test]
    fn successful_reprioritize() -> Result<(), Box<dyn Error>> {
        let priority_5_test_task = Action::create_default();

        let priority_4_test_task = &priority_5_test_task.change_priority("4")?;
        assert!(priority_4_test_task.priority == PriEnum::Low);

        let priority_3_test_task = &priority_4_test_task.change_priority("3")?;
        assert!(priority_3_test_task.priority == PriEnum::Medium);

        let priority_2_test_task = &priority_3_test_task.change_priority("2")?;
        assert!(priority_2_test_task.priority == PriEnum::High);

        let priority_1_test_task = &priority_2_test_task.change_priority("1")?;
        assert!(priority_1_test_task.priority == PriEnum::Critical);

        return Ok(());
    }



    #[test]
    fn successfully_serialize_task() {
        let test_task = Action {
            id: Uuid::nil(),
            ..Default::default()
        };

        assert_ser_tokens(
            &test_task.readable(),
            &[
                Token::Struct {
                    name: "Action",
                    len: 4,
                },
                Token::Str("name"),
                Token::Str("Default Task"),
                Token::Str("priority"),
                Token::UnitVariant {
                    name: "PriEnum",
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
    fn successfully_deserializing_task() {
        let test_task =  Action {
            id: Uuid::nil(),
            ..Default::default()
        };
    assert_de_tokens(&test_task.readable(), &[
        Token::Struct {name: "Action", len:5},
        Token::Str("name"),
        Token::Str("Default Task"),
        Token::Str("priority"),
        Token::UnitVariant {
            name: "PriEnum",
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
