use std::error::Error;

pub trait TaskManipulation {
    fn rename(&self, new_task_name: &str) -> Self;
    fn toggle_completion_status(&self) -> Self;
    fn change_priority(&self, new_priority: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn export_fields_as_string(&self) -> String;
    fn create_default_task() -> Self;
}
