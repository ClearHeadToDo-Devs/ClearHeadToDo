use crate::priority::*;
use strum::ParseError;

pub trait ActionEditing {
    fn set_name(&mut self, name: &str) -> &mut Self;
    fn set_priority(&mut self, priority_str: Priority) -> &mut Self;
    fn set_completion_status(&mut self, desired_completion_status: bool) -> &mut Self;
}

pub trait ActionViewing {
    fn get_name(&self) -> &str;
    fn get_priority(&self) -> &Priority;
    fn get_completion_status(&self) -> bool;
}
