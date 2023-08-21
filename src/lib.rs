pub mod graph_storage;
use uuid::Uuid;

#[derive(PartialEq, Clone)]
struct Action {
    id: Uuid,
    name: String,
    priority: Priority,
    completed: bool,
}

impl Action {
    fn new(name: &str, priority: Option<usize>) -> Self {
        let name = name.to_string();
        let priority: Priority = priority.unwrap_or(5).into();
        Action {
            id: Uuid::new_v4(),
            name,
            priority,
            completed: false,
        }
    }

    fn set_id(&self, new_id: Uuid) -> Action {
        Action {
            id: new_id,
            name: self.get_name(),
            priority: self.get_priority(),
            completed: self.get_completion_status(),
        }
    }
    fn rename(&self, new_name: &str) -> Action {
        Action {
            id: self.get_id(),
            name: new_name.to_string(),
            priority: self.get_priority(),
            completed: self.get_completion_status(),
        }
    }
    fn toggle_completion_status(&self) -> Self {
        Action {
            id: self.get_id(),
            name: self.get_name(),
            priority: self.get_priority(),
            completed: !self.get_completion_status(),
        }
    }
    fn set_priority(&self, new_priority: usize) -> Self {
        Action {
            id: self.get_id(),
            name: self.get_name(),
            priority: new_priority.into(),
            completed: self.get_completion_status(),
        }
    }

    fn get_id(&self) -> Uuid {
        self.id
    }
    fn get_name(&self) -> String {
        self.name.to_string()
    }
    fn get_priority(&self) -> Priority {
        self.priority
    }
    fn get_completion_status(&self) -> bool {
        self.completed
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Priority {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Optional = 5,
}

impl From<usize> for Priority {
    fn from(value: usize) -> Self {
        match value {
            1 => Priority::Critical,
            2 => Priority::High,
            3 => Priority::Medium,
            4 => Priority::Low,
            5 => Priority::Optional,
            _ => Priority::Optional,
        }
    }
}

struct Relationship {
    variant: RelationshipVariant,
    outbound: Uuid,
    inbound: Uuid,
}

impl Relationship {
    fn new(variant: &str, outbound: Uuid, inbound: Uuid) -> Self {
        Relationship {
            variant: variant.into(),
            outbound,
            inbound,
        }
    }

    fn get_variant(&self) -> RelationshipVariant {
        self.variant.clone()
    }
    fn get_outbound(&self) -> Uuid {
        self.outbound.clone()
    }
    fn get_inbound(&self) -> Uuid {
        self.inbound.clone()
    }
}

#[derive(PartialEq, Copy, Clone)]
enum RelationshipVariant {
    Hierarchical = 1,
    Sequential = 2,
    Related = 3,
}

impl From<&str> for RelationshipVariant {
    fn from(value: &str) -> Self {
        match value {
            "hierarchical" | "parental" => RelationshipVariant::Hierarchical,
            "sequential" | "sibling" => RelationshipVariant::Sequential,
            "related" | "connected" => RelationshipVariant::Related,
            _ => RelationshipVariant::Related,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    mod action {
        use super::*;

        #[rstest]
        fn create_custom(test_action: Action) {
            let constructed_action = Action::new("test", None)
                .set_id(Uuid::nil())
                .rename("renamed")
                .set_priority(1)
                .toggle_completion_status();

            assert!(constructed_action == test_action)
        }

        #[fixture]
        fn test_action() -> Action {
            Action {
                id: Uuid::nil(),
                name: "renamed".to_string(),
                priority: Priority::Critical,
                completed: true,
            }
        }
    }

    mod relationships {
        use super::*;

        #[rstest]
        fn create_default(test_relationship: Relationship) {
            assert!(
                test_relationship.get_variant() == RelationshipVariant::Hierarchical
                    && test_relationship.get_outbound().is_nil()
                    && test_relationship.get_inbound().is_nil()
            )
        }
        #[fixture]
        fn test_relationship() -> Relationship {
            Relationship::new("parental", Uuid::nil(), Uuid::nil())
        }
    }
}
