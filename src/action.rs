use uuid::Uuid;

struct ActionBuilder {
    name: String,
    completed: bool,
    priority: Priority,
    id: Uuid,
}

impl Default for ActionBuilder {
    fn default() -> Self {
        ActionBuilder {
            name: "Default Action".to_string(),
            completed: false,
            priority: Priority::Optional,
            id: Uuid::new_v4(),
        }
    }
}

#[derive(PartialEq)]
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
    fn create_action() {
        let test_builder = ActionBuilder::default();

        assert!(
            test_builder.name == "Default Action"
                && test_builder.completed == false
                && test_builder.priority == Priority::Optional
                && test_builder.id != Uuid::nil()
        )
    }
}
