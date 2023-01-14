use std::str::FromStr;
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

#[derive(PartialEq, EnumString, FromRepr)]
enum Priority {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Optional = 5,
}

#[cfg(test)]
mod test {
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
    fn create_priority_from_integer() {
        let test_priority = Priority::from_repr(1).unwrap();

        assert!(test_priority == Priority::Critical);
    }
}
