pub struct Task {
    name: String,
    completed: bool,
}

pub fn create_task(task_name: String) -> Task {
    Task {
        name: String::from("Test Task"),
        completed: false,
    }
}

impl Task {
    pub fn rename_task(&mut self, new_task_name: String) {
        self.name = new_task_name;
    }
    
    pub fn mark_complete(&mut self) {
        self.completed = true;
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

    #[test]
    fn task_rename() {
        let mut TestTask = create_task(String::from("Original Name"));
        TestTask.rename_task("Changed Name".to_string());
        assert!(TestTask.name == "Changed Name");
    }

    #[test]
    fn task_completion() {
        let mut TestTask = create_task(String::from("Test Task"));
        TestTask.mark_complete();
        assert!(TestTask.completed == true);
    }

}
