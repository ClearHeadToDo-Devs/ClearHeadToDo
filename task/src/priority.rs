use serde::Serialize as AltSerialize;
use std::error::Error;
use std::fmt;
use std::io::{Error as OtherError, ErrorKind};

#[repr(u8)]
#[derive(AltSerialize, Copy, Clone, PartialEq, Debug)]
pub enum PriEnum {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Optional = 5,
}

impl PriEnum {
    pub fn parse_priority(expr: &str) -> Result<PriEnum, Box<dyn Error>> {
        match expr.to_ascii_lowercase().trim() {
            "1" | "critical" | "crit" | "c" => Ok(PriEnum::Critical),
            "2" | "high" | "hi" | "h" => Ok(PriEnum::High),
            "3" | "medium" | "med" | "m" => Ok(PriEnum::Medium),
            "4" | "low" | "lo" | "l" => Ok(PriEnum::Low),
            "5" | "optional" | "opt" | "o" => Ok(PriEnum::Optional),
            "" => Ok(PriEnum::Optional), //defaults to this
            _ => Err(Box::new(OtherError::new(
                ErrorKind::Other,
                "invalid priority",
            ))),
        }
    }
}

impl Default for PriEnum {
    fn default() -> Self {
        PriEnum::Optional
    }
}

impl fmt::Display for PriEnum {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let printable: &str = match *self {
            PriEnum::Critical => "Critical",
            PriEnum::High => "High",
            PriEnum::Medium => "Medium",
            PriEnum::Low => "Low",
            PriEnum::Optional => "Optional",
        };
        write!(formatter, "{}", printable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default_priority() {
        let test_priority = PriEnum::default();
        assert_eq!(test_priority, PriEnum::Optional);
    }

    #[test]
    fn successfully_parse_priority() {
        let test_priority = PriEnum::parse_priority("optional").unwrap();
        assert_eq!(test_priority, PriEnum::Optional);
    }

    #[test]
    fn failed_parse_priority() {
        let test_priority_error = PriEnum::parse_priority("bad priority").unwrap_err();
        assert_eq!(
            test_priority_error.to_string(),
            "invalid priority".to_string()
        );
    }

    #[test]
    fn priority_display_test() {
        let test_priority = PriEnum::default();
        assert_eq!(test_priority.to_string(), "Optional".to_string())
    }
}
