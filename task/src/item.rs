use crate::priority::*;

use crate::action_manipulation::ActionManipulation;

use std::error::Error;
use uuid::Uuid;

use serde::Deserialize;

#[derive(PartialEq, Debug, Clone, Deserialize)]
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

    fn create_default_task() -> Action {
        Action {
            ..Default::default()
        }
    }
}
