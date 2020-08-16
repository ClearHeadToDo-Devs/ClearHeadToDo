pub struct TaskList{
tasks: Vec<Task>
}

#[derive(PartialEq)] #[derive(Debug)]
pub struct Task {
    name: String,
    completed: bool,
    priority: u8, //will be 1-5, 1 being highest
}

impl TaskList{
    pub fn create_task(&mut self){
        let new_task = Task {
            name: String::from("Test Task"),
            completed: false,
            priority: 5,
        };
        self.tasks.push(new_task);
    }
    
    pub fn print_task_list(self, mut writer: impl std::io::Write) -> std::result::Result<(), std::io::Error>
        {
        for index in 0..=self.tasks.len()-1{
            writeln!(writer, "{index}, {name}, {priority}, {completed}",
                     index = index,
                     name = self.tasks[index].name,
                     priority = self.tasks[index].priority,
                     completed = self.tasks[index].completed)?; 
        }
        Ok(())
    }
    
    pub fn select_task(&mut self, index: usize) -> &Task 
    {
        if index < self.tasks.len(){
            return &self.tasks[index];
        }
        else{panic!("can't do that index number!")};
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
        let mut test_task_list = TaskList{tasks: vec![]}; 
        test_task_list.create_task();
        let test_task = &test_task_list.tasks[0];
        assert!(test_task.name == "Test Task");
        assert!(test_task.completed == false);
        assert!(test_task.priority == 5);
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
        test_task.change_priority(4);
        assert!(test_task.priority == 4);
        test_task.change_priority(3);
        assert!(test_task.priority == 3);
        test_task.change_priority(2);
        assert!(test_task.priority == 2);
        test_task.change_priority(1);
        assert!(test_task.priority == 1);
        test_task.change_priority(6);
        assert!(test_task.priority == 1); //should NOT change when invalid val

    }
    
    #[test]
    fn task_print_test(){
        let mut test_task_list = TaskList{tasks: vec![]}; 
        test_task_list.create_task();
        let mut result = Vec::new();
        test_task_list.print_task_list(&mut result).unwrap();
        assert_eq!(&result[..], "0, Test Task, 5, false\n".as_bytes());
        println!("{:?}", result);
    }

    #[test]
    fn task_selection_test(){
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task();
        let test_selection_task = test_task_list.select_task(0);
        assert_eq!(test_selection_task.name, "Test Task".to_string());
    }
}




