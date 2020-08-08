pub struct Task {
    name: String,
    completed: bool,
    priority: u8, //will be 1-5, 1 being highest
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
        if new_priority < 1 || new_priority > 5 {
            println!("Invalid priority: enter a number between 1 and 5,\
                     with 1 being highest priority");
        }
        else {
            self.priority = new_priority;
        }
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
        TestTask.change_priority(3);
        assert!(TestTask.priority == 3);
        TestTask.change_priority(2);
        assert!(TestTask.priority == 2);
        TestTask.change_priority(1);
        assert!(TestTask.priority == 1);
        TestTask.change_priority(6);
        assert!(TestTask.priority == 1); //should NOT change when invalid val

    }

}
