pub mod item;
pub use item::*;

pub mod error;
pub use error::*;

use std::error::Error;
use uuid::Uuid;
use im::Vector;

pub trait ActionListManipulation {
    fn create_new(&self) -> Self;

    fn rename(&self, index: usize, new_name: String) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn toggle_completion_status(&self, index: usize) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn change_priority(
        &self,
        index: usize,
        new_priority: String,
    ) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    fn select_by_id(&self, id: Uuid) -> Result<Action, Box<dyn Error>>;
    fn select_by_index(&self, index: usize) -> Result<Action, Box<dyn Error>>;
    fn get_action_name(&self, index: usize) -> Result<String, Box<dyn Error>>;
    fn get_action_priority(&self, index: usize) -> Result<Priority, Box<dyn Error>>;
    fn get_action_completion_status(&self, index: usize) -> Result<bool, Box<dyn Error>>;
    fn get_action_id(&self, index: usize) -> Result<Uuid, Box<dyn Error>>;

    fn remove_action(&self, index: usize) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

impl ActionListManipulation for Vector<Action> {
    fn create_new(&self) -> Self {
        let mut new_list = self.clone();

        new_list.push_back(Action::default());

        return new_list;
    }

    fn remove_action(&self, index: usize) -> Result<Vector<Action>, Box<dyn Error>> {
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
        let cloned_list = self.clone();

        let updated_action = self.select_by_index(index)?.rename(&new_name);

        Ok(cloned_list.update(index, updated_action))
    }

    fn toggle_completion_status(
        &self,
        index: usize,
    ) -> Result<im::Vector<Action>, Box<dyn Error>> {
        let cloned_list = self.clone();

        let updated_action = self.select_by_index(index)?.toggle_completion_status();

        Ok(cloned_list.update(index, updated_action))
    }

    fn change_priority(
        &self,
        index: usize,
        new_priority: String,
    ) -> Result<im::Vector<Action>, Box<dyn Error>> {
        let cloned_list = self.clone();

        let updated_action = self.select_by_index(index)?
            .change_priority(&new_priority)?;

        Ok(cloned_list.update(index, updated_action))
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

    fn select_by_index(&self, index: usize) -> Result<Action, Box<dyn Error>> {
        match self.iter().nth(index) {
            Some(action_ref) => return Ok(action_ref.clone()),
            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }

    fn get_action_id(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
        match self.iter().nth(index) {
            Some(action_ref) => return Ok(action_ref.id.clone()),
            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }

    fn get_action_name(&self, index: usize) -> Result<String, Box<dyn Error>> {
        match self.iter().nth(index) {
            Some(action_ref) => return Ok(action_ref.get_name()),
            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }

    fn get_action_priority(&self, index: usize) -> Result<Priority, Box<dyn Error>> {
        match self.iter().nth(index) {
            Some(action_ref) => return Ok(action_ref.get_priority()),
            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }
    fn get_action_completion_status(&self, index: usize) -> Result<bool, Box<dyn Error>> {
        match self.iter().nth(index) {
            Some(action_ref) => return Ok(action_ref.get_completion_status()),
            None => Err(ActionError::InvalidIndex(index).into()),
        }
    }
}
