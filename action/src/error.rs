use std::fmt::Display;

#[derive(Debug)]
pub enum ActionError {
    InvalidPriority(String),
    InvalidIndex(usize),
}

impl Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionError::InvalidPriority(bad_priority) => 
                write!(f, "{} is an Invalid Priority Option", bad_priority),
            ActionError::InvalidIndex(bad_index) => 
                write!(f, "No Action at Index {}", bad_index),
        }
    }
}

// Make it an error!
impl std::error::Error for ActionError {}
