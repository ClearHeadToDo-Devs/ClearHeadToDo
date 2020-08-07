pub struct Task {
    name: String
}

pub fn create_task(task_name: String) -> Task {
    Task {
        name: String::from("Fail Name")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_creation() {
        let TestTask = create_task(String::from("Test Task"));
        assert!(TestTask.name == "Test Task")
    }
}
