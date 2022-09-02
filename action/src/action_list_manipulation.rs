use crate::Action;
use std::error::Error;
use uuid::Uuid;


pub trait ActionListManipulation {
    fn create_new(&self) -> Self;
    fn remove(&self, index: usize) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
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
    fn get_id_by_index(&self, index: usize) -> Result<Uuid, Box<dyn Error>>;
}
