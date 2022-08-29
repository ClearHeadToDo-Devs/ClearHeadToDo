pub mod priority;
pub use priority::*;

pub mod action_manipulation;
pub use action_manipulation::ActionManipulation;

pub mod item;
pub use item::*;

pub mod helper;
pub use helper::*;

pub mod storage;
pub use storage::*;

pub mod action_list_manipulation;
pub use action_list_manipulation::ActionListManipulation;

pub mod error;
pub use error::*;

use std::error::Error;
use std::io::{Error as OtherError, ErrorKind};
use uuid::Uuid;

impl ActionListManipulation for im::Vector<Action> {
    type Child = Action;

    fn create_new(&self) -> Self {
        let mut new_list = self.clone();
        new_list.push_back(Action::create_default());

        return new_list;
    }

    fn print_list(&self) -> Result<String, Box<dyn Error>> {
        let mut task_list_string = "name,priority,completed,ID\n".to_string();

        if self.is_empty() == true {
            return Err(Box::new(OtherError::new(ErrorKind::Other, "list is empty")));
        } else {
            for task in self {
                task_list_string.push_str(&task.export_fields_as_string());
            }
        }

        Ok(task_list_string.to_owned())
    }

    fn remove(&self, index: usize) -> Result<im::Vector<Action>, Box<dyn Error>> {
        match self.iter().nth(index) {
            Some(_task_ref) => {
                let (mut left_side, mut right_side) = self.clone().split_at(index);
                right_side.pop_front().unwrap();
                left_side.append(right_side);
                Ok(left_side)
            }
            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }

    fn rename(
        &self,
        index: usize,
        new_name: String,
    ) -> Result<im::Vector<Action>, Box<dyn Error>> {
        match self.iter().nth(index) {
            Some(task_ref) => return Ok(self.update(index, task_ref.rename(&new_name))),

            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }

    fn toggle_completion_status(
        &self,
        index: usize,
    ) -> Result<im::Vector<Action>, Box<dyn Error>> {
        match self.iter().nth(index) {
            Some(task_ref) => Ok(self.update(index, task_ref.clone().toggle_completion_status())),
            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }

    fn change_priority(
        &self,
        index: usize,
        new_priority: String,
    ) -> Result<im::Vector<Action>, Box<dyn Error>> {
        match self.iter().nth(index) {
            Some(task_ref) => {
                Ok(self.update(index, task_ref.clone().change_priority(&new_priority)?))
            }
            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }

    fn select_by_id(&self, id: Uuid) -> Result<Action, Box<dyn Error>> {
        let search_task = self.clone().into_iter().find(|tasks| tasks.id == id);
        match search_task {
            Some(task) => return Ok(task.clone().to_owned()),
            None => {
                return Err(ActionError::InvalidId(id).into())
            }
        }
    }
}
