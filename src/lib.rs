#[repr(u8)]
#[derive(PartialEq)]
pub enum PriEnum {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Optional = 5,
}

pub fn parse_priority(expr: &str) -> Result<PriEnum, String> {
    match expr.to_ascii_lowercase().trim() {
        "1" | "critical" | "crit" | "c" => Ok(PriEnum::Critical),
        "2" | "high" | "hi" | "h" => Ok(PriEnum::High),
        "3" | "medium" | "med" | "m" => Ok(PriEnum::Medium),
        "4" | "low" | "lo" | "l" => Ok(PriEnum::Low),
        "5" | "optional" | "opt" | "o" => Ok(PriEnum::Optional),
        "" => Ok(PriEnum::Optional), //defaults to this
        _ => Err(format!("Invalid priority value")),
    }
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
    
    pub fn change_priority(&mut self, new_priority: &str) {
        let new_pri = parse_priority(new_priority);
        match new_pri {
            Ok(i) => self.priority = i,
            _ => (),
        };
        
    }
   
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
        assert!(test_task.priority == PriEnum::Critical); //should NOT change on invalid input
    }
    

}
