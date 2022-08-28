use crate::action_manipulation::ActionManipulation;
use std::error::Error;
use uuid::Uuid;
pub trait ActionListManipulation {
    type Child: ActionManipulation;
    fn create_new(&self) -> Self;
    fn print_list(&self) -> Result<String, Box<dyn Error>>;
    fn remove_task(&self, index: usize) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn rename_task(&self, index: usize, new_name: String) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn toggle_task_completion_status(&self, index: usize) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn change_task_priority(
        &self,
        index: usize,
        new_priority: String,
    ) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn select_task_by_id(&self, id: Uuid) -> Result<Self::Child, Box<dyn Error>>;
}
