use std::str::FromStr;
use strum::*;
use strum_macros::*;

struct ActionBuilder {
    name: String,
    completed: bool,
    priority: Priority,
}

impl ActionBuilder {
    fn set_name(self: &mut Self, new_name: &str) {
        self.name = new_name.to_string()
    }

    fn set_priority(self: &mut Self, priority_str: &str) -> Result<(), ParseError> {
        self.priority = Priority::from_str(priority_str)?;

        Ok(())
    }

    fn set_completion_status(self: &mut Self, desired_status: bool) {
        self.completed = desired_status;
    }
}

impl Default for ActionBuilder {
    fn default() -> Self {
        ActionBuilder {
            name: "Default Action".to_string(),
            completed: false,
            priority: Priority::Optional,
        }
    }
}

#[derive(PartialEq, EnumString, FromRepr, Debug)]
enum Priority {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Optional = 5,
}

#[cfg(test)]
mod test {
    use strum::ParseError;

    use super::*;
    #[test]
    fn create_default_builder() {
        let test_builder = ActionBuilder::default();

        assert!(
            test_builder.name == "Default Action"
                && test_builder.completed == false
                && test_builder.priority == Priority::Optional
        )
    }

    #[test]
    fn update_builder_name() {
        let mut test_builder = ActionBuilder::default();

        test_builder.set_name("New Name");

        assert!(test_builder.name == "New Name")
    }

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

    #[test]
    fn update_builder_priority() {
        let mut test_builder = ActionBuilder::default();

        test_builder.set_priority("Optional").unwrap();

        assert!(test_builder.priority == Priority::Optional);
    }

    #[test]
    fn failed_update_builder_priority() {
        let mut test_builder = ActionBuilder::default();

        let failure_message = test_builder.set_priority("Bad Priority").unwrap_err();

        assert!(failure_message == ParseError::VariantNotFound);
    }

    #[test]
    fn update_builder_completion_status() {
        let mut test_builder = ActionBuilder::default();

        test_builder.set_completion_status(true);

        assert!(test_builder.completed == true);
    }
}
