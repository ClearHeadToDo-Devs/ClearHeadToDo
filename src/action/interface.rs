use crate::Priority;
use uuid::Uuid;

pub trait ActionEditing {
    fn set_name(&mut self, name: &str) -> &mut Self;
    fn set_priority(&mut self, priority_str: Priority) -> &mut Self;
    fn set_completion_status(&mut self, desired_completion_status: bool) -> &mut Self;
    fn set_id(&mut self, id: Uuid) -> &mut Self;
}

pub trait ActionViewing {
    fn get_name(&self) -> &str;
    fn get_priority(&self) -> &Priority;
    fn get_completion_status(&self) -> bool;
    fn get_id(&self) -> Uuid;
}
