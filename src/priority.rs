use strum_macros::*;
use strum::*;

use std::str::FromStr;

#[derive(PartialEq, EnumString, FromRepr, Debug, Clone, Copy, Display)]
pub enum Priority {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Optional = 5,
}

#[cfg(test)]
mod priority {
    use super::*;

    #[test]
    fn create_priority_from_string() {
        let test_priority = Priority::from_str("Critical").unwrap();

        assert!(test_priority == Priority::Critical);
    }

    #[test]
    fn failed_created_priority_from_string() {
        let priority_conversion_error = Priority::from_str("Bad Priority").unwrap_err();

        assert!(priority_conversion_error == ParseError::VariantNotFound)
    }

    #[test]
    fn create_priority_from_integer() {
        let test_priority = Priority::from_repr(1).unwrap();

        assert!(test_priority == Priority::Critical);
    }

    #[test]
    fn failed_create_priority_from_integer() {
        let priority_conversion_error = Priority::from_repr(6)
            .ok_or("Invalid Priority Selection")
            .unwrap_err();

        assert!(priority_conversion_error == "Invalid Priority Selection");
    }
}
