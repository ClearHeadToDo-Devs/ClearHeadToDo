use std::fmt::Display;

#[derive(Debug)]
pub enum ActionError {
    InvalidPriority(String),

}

impl Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionError::InvalidPriority(bad_priority) => 
                write!(f, "{} is an Invalid Priority Option", bad_priority),
        }
    }
}

// Make it an error!
impl std::error::Error for ActionError {}
