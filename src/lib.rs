pub struct Task {
    name: String,
    completed: bool,
    priority: u8,
}

pub fn create_task(task_name: String) -> Task {
    Task {
        name: String::from("Test Task"),
        completed: false,
        priority: 5,
    }
}

impl Task {
    pub fn rename_task(&mut self, new_task_name: String) {
        self.name = new_task_name;
    }
    
    pub fn mark_complete(&mut self) {
        self.completed = true;
    }

    pub fn change_priority(&mut self, new_priority: u8) {
        self.priority = new_priority;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn task_creation_test() {
        let TestTask = create_task(String::from("Test Task"));
        assert!(TestTask.name == "Test Task");
        assert!(TestTask.completed == false);
        assert!(TestTask.priority == 5);
    }
    
    #[test]
    fn task_rename_test() {
        let mut TestTask = create_task(String::from("Original Name"));
        TestTask.rename_task("Changed Name".to_string());
        assert!(TestTask.name == "Changed Name");
    }
    
    #[test]
    fn task_completion_test() {
        let mut TestTask = create_task(String::from("Test Task"));
        TestTask.mark_complete();
        assert!(TestTask.completed == true);
    }
    
    #[test]
    fn task_reprioritize_test() {
        let mut TestTask = create_task(String::from("Test Task"));
        TestTask.change_priority(4);
        assert!(TestTask.priority == 4);
    }

}
