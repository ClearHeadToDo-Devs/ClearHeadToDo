use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug)]
pub enum ActionError {
    InvalidPriority(String),
    InvalidIndex(usize),
    InvalidId(Uuid),
}

impl Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionError::InvalidPriority(bad_priority) => 
                write!(f, "{} is an Invalid Priority Option", bad_priority),
            ActionError::InvalidIndex(bad_index) => 
                write!(f, "No Action at Index {}", bad_index),
            ActionError::InvalidId(bad_id) => 
                write!(f, "No Action with Id {}", bad_id),
        }
    }
}

// Make it an error!
impl std::error::Error for ActionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_priority() {
        let error = ActionError::InvalidPriority("Bad Priority".to_string());
        assert_eq!(error.to_string(), "Bad Priority is an Invalid Priority Option");
    }

    #[test]
    fn test_invalid_index() {
        let error = ActionError::InvalidIndex(5);
        assert_eq!(error.to_string(), "No Action at Index 5");
    }

    #[test]
    fn test_invalid_id() {
        let error = ActionError::InvalidId(Uuid::new_v4());
        assert_eq!(error.to_string().contains("No Action with Id"), true);
    }
}
