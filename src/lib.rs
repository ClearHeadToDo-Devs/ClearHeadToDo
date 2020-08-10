#[repr(u8)]
#[derive(PartialEq)]
pub enum PriEnum {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Optional = 5,
}

pub struct Task {
    name: String,
    completed: bool,
    priority: PriEnum, 
}

pub fn create_task() -> Task {
    Task {
        name: String::from("Test Task"),
        completed: false,
        priority: PriEnum::Optional,
    }
}

impl Task {
    pub fn rename_task(&mut self, new_task_name: String) {
        self.name = new_task_name;
    }
    
    pub fn mark_complete(&mut self) {
        self.completed = true;
    }
    /*
    pub fn change_priority(&mut self, new_priority: u8) {
        if new_priority < 1 || new_priority > 5 {
            println!("Invalid priority: enter a number between 1 and 5,\
                     with 1 being highest priority");
        }
        else {
            self.priority = new_priority;
        }
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn task_creation_test() {
        let test_task = create_task();
        assert!(test_task.name == "Test Task");
        assert!(test_task.completed == false);
        assert!(test_task.priority == PriEnum::Optional);
    }
    
    #[test]
    fn task_rename_test() {
        let mut test_task = create_task();
        test_task.rename_task("Changed Name".to_string());
        assert!(test_task.name == "Changed Name");
    }
    
    #[test]
    fn task_completion_test() {
        let mut test_task = create_task();
        test_task.mark_complete();
        assert!(test_task.completed == true);
    }
    /*
    #[test]
    fn task_reprioritize_test() {
        let mut test_task = create_task();
        test_task.change_priority("4");
        assert!(test_task.priority == PriEnum::Low);
        test_task.change_priority("3");
        assert!(test_task.priority == PriEnum::Medium);
        test_task.change_priority("2");
        assert!(test_task.priority == PriEnum::High);
        test_task.change_priority("1");
        assert!(test_task.priority == PriEnum::Critical);
        test_task.change_priority("6");
        assert!(test_task.priority == PriEnum::Critical); //should NOT change when invalid val
    }
    */

}
