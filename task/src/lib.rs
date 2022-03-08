pub mod priority;
pub use priority::*;

pub mod task_manipulation;
pub use task_manipulation::TaskManipulation;

pub mod storage;
pub use storage::*;

use std::error::Error;
use uuid::Uuid;

use serde::Deserialize;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub completed: bool,
    pub priority: PriEnum,
}

impl Default for Task {
    fn default() -> Task {
        Task {
            id: Uuid::new_v4(),
            name: "Default Task".to_string(),
            completed: false,
            priority: Default::default(),
        }
    }
}

impl TaskManipulation for Task {
    fn rename(&self, new_task_name: &str) -> Task {
        return Task {
            name: new_task_name.to_owned(),
            ..self.to_owned()
        };
    }
    fn toggle_completion_status(&self) -> Task {
        Task {
            completed: !self.completed,
            ..self.to_owned()
        }
    }

    fn change_priority(&self, new_priority: &str) -> Result<Task, Box<dyn Error>> {
        let new_pri: PriEnum = PriEnum::parse_priority(new_priority)?;
        return Ok(Task {
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

    fn create_default_task() -> Task {
        Task {
            ..Default::default()
        }
    }
}
