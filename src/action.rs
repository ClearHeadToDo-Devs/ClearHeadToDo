use uuid::Uuid;

struct ActionBuilder {
    name: String,
    completed: bool,
    priority: Priority,
    id: Uuid
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
        let test_builder = ActionBuilder {
            name: "Default Builder".to_string(),
            completed: false,
            priority: Priority::Optional,
            id: Uuid::new_v4(),
        };

        assert!(
            test_builder.name == "Default Builder"
                && test_builder.completed == false
                && test_builder.priority == Priority::Optional
        )
    }
}
