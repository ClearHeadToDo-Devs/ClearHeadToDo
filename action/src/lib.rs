pub mod item;
pub use item::*;

pub mod storage;
pub use storage::*;

pub mod error;
pub use error::*;

pub mod action_list_manipulation;
pub use action_list_manipulation::*;

use std::error::Error;
use std::io::{Error as OtherError, ErrorKind};
use uuid::Uuid;

impl ActionListManipulation for im::Vector<Action> {
    type Child = Action;

    fn create_new(&self) -> Self {
        let mut new_list = self.clone();
        new_list.push_back(Action::default());

        return new_list;
    }

    fn get_list(&self) -> Result<String, Box<dyn Error>> {
        let mut action_list_string = "order,name,priority,completed,ID\n".to_string();
        let mut index = 0;

        if self.is_empty() == true {
            return Err(Box::new(OtherError::new(ErrorKind::Other, "list is empty")));
        } else {
            for action in self {
                action_list_string.push_str(&format!("{},{}",index,&action.to_string()));
                index += 1;
            }
        }

        Ok(action_list_string.to_owned())
    }

    fn remove(&self, index: usize) -> Result<im::Vector<Action>, Box<dyn Error>> {
        match self.iter().nth(index) {
            Some(_action_ref) => {
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
            Some(action_ref) => return Ok(self.update(index, action_ref.rename(&new_name))),

            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }

    fn toggle_completion_status(
        &self,
        index: usize,
    ) -> Result<im::Vector<Action>, Box<dyn Error>> {
        match self.iter().nth(index) {
            Some(action_ref) => Ok(self.update(index, action_ref.clone().toggle_completion_status())),
            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }

    fn change_priority(
        &self,
        index: usize,
        new_priority: String,
    ) -> Result<im::Vector<Action>, Box<dyn Error>> {
        match self.iter().nth(index) {
            Some(action_ref) => {
                Ok(self.update(index, action_ref.clone().change_priority(&new_priority)?))
            }
            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }

    fn select_by_id(&self, id: Uuid) -> Result<Action, Box<dyn Error>> {
        let search_action = self.clone().into_iter().find(|actions| actions.id == id);
        match search_action {
            Some(action) => return Ok(action.clone().to_owned()),
            None => {
                return Err(ActionError::InvalidId(id).into())
            }
        }
    }

    fn get_id_by_index(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        match self.iter().nth(index) {
            Some(action_ref) => return Ok(action_ref.id.clone()),
            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }
}
