use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::error::ActionError;

#[repr(u8)]
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub enum Priority {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Optional = 5,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Optional
    }
}

impl FromStr for Priority {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "1" | "critical" | "crit" | "c" => Ok(Priority::Critical),
            "2" | "high" | "hi" | "h" => Ok(Priority::High),
            "3" | "medium" | "med" | "m" => Ok(Priority::Medium),
            "4" | "low" | "lo" | "l" => Ok(Priority::Low),
            "5" | "optional" | "opt" | "o" => Ok(Priority::Optional),
            "" => Ok(Priority::Optional), //defaults to this
            _ => Err(ActionError::InvalidPriority(s.to_owned()).into()),
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let printable: &str = match *self {
            Priority::Critical => "Critical",
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
            Priority::Optional => "Optional",
        };
        write!(formatter, "{}", printable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    #[test]
    fn create_default_priority() {
        let test_priority = Priority::default();
        assert_eq!(test_priority, Priority::Optional);
    }

    #[test]
    fn successfully_parse_priority() {
        let test_priority = Priority::from_str("optional").unwrap();
        assert_eq!(test_priority, Priority::Optional);
    }

    #[test]
    fn failed_parse_priority() {
        let test_priority_error = Priority::from_str("bad priority").unwrap_err();
        assert_eq!(
            test_priority_error.to_string(),
            "bad priority is an Invalid Priority Option".to_string()
        );
    }

    #[test]
    fn priority_display_test() {
        let test_priority = Priority::default();
        assert_eq!(test_priority.to_string(), "Optional".to_string())
    }

    #[test]
    fn successfully_serialize_action() {
        let test_priority = Priority::default();

        assert_ser_tokens(
            &test_priority,
            &[
                Token::UnitVariant {
                    name: "Priority",
                    variant: "Optional",
                },
            ],
        );
    }

    #[test]
    fn successfully_deserializing_action() {
        let test_priority = Priority::default();

        assert_de_tokens(
            &test_priority,
            &[
                Token::UnitVariant {
                    name: "Priority",
                    variant: "Optional",
                },
            ],
        );
    }
}
