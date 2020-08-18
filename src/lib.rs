use std::fmt;
use std::io::{Error, ErrorKind}; 

pub struct TaskList{
    tasks: Vec<Task>
}

#[derive(PartialEq)]
#[derive(Debug)]
#[repr(u8)]
pub enum PriEnum {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Optional = 5,
}

impl fmt::Display for PriEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            PriEnum::Critical => "Critical",
            PriEnum::High => "High",
            PriEnum::Medium => "Medium",
            PriEnum::Low => "Low",
            PriEnum::Optional => "Optional"
        };
        write!(f, "{}", printable)
    }
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

#[derive(PartialEq, Debug)]
pub struct Task {
    name: String,
    completed: bool,
    priority: PriEnum, 
}

impl TaskList {
    

    
    pub fn create_task(&mut self) {
        let new_task = Task {
            name: String::from("Test Task"),
            completed: false,
            priority: PriEnum::Optional,
        };
        self.tasks.push(new_task);
    }
    
    pub fn print_task_list(self, mut writer: impl std::io::Write)->
                                        Result<(), std::io::Error> {
        if self.tasks.is_empty()==true{
            return Err(Error::new(ErrorKind::Other, "list is empty"));
        } else{
            for index in 0..=self.tasks.len()-1 {
                writeln!(writer, "{index}, {name}, {priority}, {completed}",
                        index = index,
                        name = self.tasks[index].name,
                        priority = self.tasks[index].priority,
                        completed = self.tasks[index].completed)?;
            }
        }
        Ok(())
    }
    
    pub fn select_task(&mut self, index: usize) -> Result<&Task, String> {
        if index < self.tasks.len() {
            return Ok(&self.tasks[index]);
        }
        else {
            return Err("can't do that index number!".to_string());
        }
    }

} //end 'impl TaskList'
 

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
            Err(err) => println!("{}", err),
        };
        
    }
   
} //end 'impl Task'

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn task_creation_test() {
        let mut test_task_list = TaskList{tasks: vec![]}; 
        test_task_list.create_task();
        let test_task = &test_task_list.tasks[0];
        assert!(test_task.name == "Test Task");
        assert!(test_task.completed == false);
        assert!(test_task.priority == PriEnum::Optional);
        assert!(&test_task_list.tasks[0] == test_task);
    }
    
    #[test]
    fn task_rename_test() {
        let mut test_task_list = TaskList{tasks: vec![]}; 
        test_task_list.create_task();         
        let test_task = &mut test_task_list.tasks[0];
        test_task.rename_task("Changed Name".to_string());
        assert!(test_task.name == "Changed Name");
    }
    
    #[test]
    fn task_completion_test() {
        let mut test_task_list = TaskList{tasks: vec![]}; 
        test_task_list.create_task();
        let test_task = &mut test_task_list.tasks[0];
        test_task.mark_complete();
        assert!(test_task.completed == true);
    }
    
    #[test]
    fn task_reprioritize_test() {
        let mut test_task_list = TaskList{tasks: vec![]}; 
        test_task_list.create_task();
        let test_task = &mut test_task_list.tasks[0];
        test_task.change_priority("4");
        assert!(test_task.priority == PriEnum::Low);
        test_task.change_priority("3");
        assert!(test_task.priority == PriEnum::Medium);
        test_task.change_priority("2");
        assert!(test_task.priority == PriEnum::High);
        test_task.change_priority("1");
        assert!(test_task.priority == PriEnum::Critical);
        test_task.change_priority("6");
        assert!(test_task.priority == PriEnum::Critical); //should NOT change on bad input
    }
    
    #[test]
    fn task_print_fail_test(){
        let test_task_list = TaskList{tasks: vec![]};
        let mut bad_result = Vec::new();
        let error = test_task_list.print_task_list(&mut bad_result).unwrap_err();
        assert_eq!(error.to_string(), "list is empty");
    }
    #[test]
    fn task_print_full_test(){
        let mut test_task_list = TaskList{tasks: vec![]};
        let mut good_result = Vec::new();
        test_task_list.create_task();
        test_task_list.print_task_list(&mut good_result).unwrap();
        assert_eq!(&good_result[..], "0, Test Task, Optional, false\n".as_bytes());
    }
    
    #[test]
    fn task_selection_test(){
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task();
        let test_selection_task = test_task_list.select_task(0).unwrap();
        assert_eq!(test_selection_task.name, "Test Task".to_string());
        let failed_selection = test_task_list.select_task(1).unwrap_err();
        assert_eq!(failed_selection, "can't do that index number!"); 
    }
}


