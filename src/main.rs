use clear_head_todo::PriEnum;
use clear_head_todo::TaskList;
use std::error::Error;

#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("ClearHeadToDo")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(
            App::new("List Tasks")
                .about("Creates new file in Task List"))
        .get_matches();


    if matches.is_present("List Tasks") {
        println!("Listing all Tasks");
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn cli_creation_test() {
        let test_matches = App::new("Test App")
            .author(crate_authors!());
        assert_eq!(test_matches.p.meta.author.unwrap() , "Mantis-Shrimp <dargondab9@gmail.com>");
    }

}
//pub struct Cli {
//    pub pattern: Option<String>,
//    pub index: Option<String>,
//    pub input: Option<String>,
//    pub task_vec: TaskList,
//}

//impl Cli {
//    pub fn parse_arguments(&mut self) -> Result<String, Box<dyn Error>> {
//        let index = |index: &Option<String>|{
//            index.as_ref().unwrap().to_string().parse::<usize>().unwrap()};
//        match self.pattern.as_ref().unwrap_or(&"None".to_string()) as &str {
//            "create_task" | "create" | "ct" | "new_task" | "new" => self.task_vec.create_task(),
//            "list_tasks" | "lt" | "list" | "list_all" => {
//                self.task_vec.print_task_list(std::io::stdout())
//            }
//            "remove_task" | "remove" | "rt" | "delete_task" | "delete" => {
//                self.task_vec.remove_task(index(&self.index))
//            }
//            "complete_task" | "complete" | "mark_complete" => {
//                self.task_vec.tasks[index(&self.index)].mark_complete()
//            }
//            "change_priority" | "cp" | "new_priority" | "np" => self.task_vec.tasks
//                [index(&self.index)]
//            .change_priority(&self.input.as_ref().unwrap()[..]),
//            "rename_task" | "rename" | "name" | "r" => {
//                self.task_vec.tasks[index(&self.index)].rename_task(self.input.as_ref().unwrap())
//            }
//            "help" | "commands" | "h" | "cmds" => {
//                return Ok(
//                "Here are the list of commands you can use \n\
//                create task: create a default task with default configurations \n\
//                list_task: list all existing tasks with all state information \n\
//                complete_task: given an index, sets the task to complete \n\
//                change_priority: given an index and a new priority {{Critical, \
//                    high, medium, low, optional}} the selected task will be changed \n\
//                rename_task: given and index and a new name, change task {{\
//                    REMEMBER, new name must be surrounded by \" to get more than\
//                    1 word }} \n\
//                remove task: given an index, remove the task from the list forever.".to_string());
//            }
//            _ => return Ok("Try putting in a command to see what we can do!".to_string()),
//        }
//    }
//}

//fn main() {
//    println!("Hello, Welcome to ClearHead ToDo!");

//    let mut main_cli: Cli = Cli {
//        pattern: std::env::args().nth(1),
//        index: std::env::args().nth(2),
//        input: std::env::args().nth(3),
//        task_vec: TaskList { tasks: vec![] },
//    };

//    main_cli.task_vec.load_tasks("tasks.csv").unwrap();

//    match main_cli.parse_arguments() {
//        Ok(output) => println!("{}", output),
//        Err(error) => println!("{}", error),
//    };

//    main_cli.task_vec.load_csv("tasks.csv").unwrap();
//}

//#[cfg(test)]
//mod tests {
//    use super::*;

//    #[test]
//    fn cli_creation_test() {
//        let test_cli = Cli {
//            pattern: None,
//            index: None,
//            input: None,
//            task_vec: TaskList { tasks: vec![] },
//        };
//        assert!(test_cli.pattern == None);
//        assert!(test_cli.index == None);
//        assert!(test_cli.input == None);
//        assert!(test_cli.task_vec.tasks.len() == 0);
//    }

//    #[test]
//    fn cli_blank_test() {
//        let mut test_cli = Cli {
//            pattern: None,
//            index: None,
//            input: None,
//            task_vec: TaskList { tasks: vec![] },
//        };
//        let result = test_cli.parse_arguments().unwrap();
//        assert_eq!(result, "Try putting in a command to see what we can do!");
//    }

//    #[test]
//    fn cli_task_creation_test() {
//        let mut test_cli = Cli {
//            pattern: Some("create_task".to_string()),
//            index: None,
//            input: None,
//            task_vec: TaskList { tasks: vec![] },
//        };
//        test_cli.parse_arguments().unwrap();
//        assert!(test_cli.task_vec.tasks.len() == 1);
//        assert!(test_cli.task_vec.tasks[0].name == "Test Task");
//        assert!(test_cli.task_vec.tasks[0].completed == false);
//        assert!(test_cli.task_vec.tasks[0].priority == PriEnum::Optional);
//    }

//    #[test]
//    fn cli_task_list_successful_test() {
//        //let mut good_result = Vec::new();
//        let mut test_cli = Cli {
//            pattern: Some("list_tasks".to_string()),
//            index: None,
//            input: None,
//            task_vec: TaskList { tasks: vec![] },
//        };
//        test_cli.task_vec.create_task().unwrap();
//        let response: String = test_cli.parse_arguments().unwrap();
//        assert_eq!(response, "Successfully Printed {}");
//    }

//    #[test]
//    fn cli_task_list_failure_test() {
//        //let mut good_result = Vec::new();
//        let mut test_cli = Cli {
//            pattern: Some("list_tasks".to_string()),
//            index: None,
//            input: None,
//            task_vec: TaskList { tasks: vec![] },
//        };
//        let response: String = test_cli.parse_arguments().unwrap_err().to_string();
//        assert_eq!(response, "list is empty");
//    }

//    #[test] 
//    fn cli_task_removal_successful_test() {
//        let mut test_cli = Cli {
//            pattern: Some("remove_task".to_string()),
//            index: Some("0".to_string()),
//            input: None,
//            task_vec: TaskList { tasks: vec![] },
//        };
//        test_cli.task_vec.create_task().unwrap();
//        let response = test_cli.parse_arguments().unwrap();
//        assert!(test_cli.task_vec.tasks.is_empty());
//        assert_eq!(response, "Successfully Removed Task Test Task");
//    }
    
//    #[test] 
//    fn cli_task_removal_failure_test() {
//        let mut test_cli = Cli {
//            pattern: Some("remove_task".to_string()),
//            index: Some("0".to_string()),
//            input: None,
//            task_vec: TaskList { tasks: vec![] },
//        };
//        let error = test_cli.parse_arguments().unwrap_err();
//        assert_eq!(error.to_string(), "Invalid Index for Deletion");
//    }

//    #[test] 
//    fn cli_task_completion_successful_test() {
//        let mut test_cli = Cli {
//            pattern: Some("complete".to_string()),
//            index: Some("0".to_string()),
//            input: None,
//            task_vec: TaskList { tasks: vec![] },
//        };
//        test_cli.task_vec.create_task().unwrap();
//        let response = test_cli.parse_arguments().unwrap();
//        assert!(test_cli.task_vec.tasks[0].completed == true);
//        assert_eq!(response, "completed Task: Test Task");
//    }

//    #[test] 
//    fn cli_task_completion_failure_test() {
//        let mut test_cli = Cli {
//            pattern: Some("complete".to_string()),
//            index: Some("0".to_string()),
//            input: None,
//            task_vec: TaskList { tasks: vec![] },
//        };
//        test_cli.task_vec.create_task().unwrap();
//        test_cli.task_vec.tasks[0].mark_complete().unwrap();
//        let error = test_cli.parse_arguments().unwrap_err().to_string();
//        assert!(test_cli.task_vec.tasks[0].completed == true);
//        assert_eq!(error, "Task is already completed");
//    }

//    #[test] 
//    fn cli_task_reprioritize_successful_test() {
//        let mut test_cli = Cli {
//            pattern: Some("cp".to_string()),
//            index: Some("0".to_string()),
//            input: Some("high".to_string()),
//            task_vec: TaskList { tasks: vec![] },
//        };
//        test_cli.task_vec.create_task().unwrap();
//        let response = test_cli.parse_arguments().unwrap();
//        assert!(test_cli.task_vec.tasks[0].priority == PriEnum::High);
//        assert_eq!(response, "changed Task: Test Task priority changed to High");
//    }

//    #[test] 
//    fn cli_task_reprioritize_failure_test() {
//        let mut test_cli = Cli {
//            pattern: Some("cp".to_string()),
//            index: Some("0".to_string()),
//            input: Some("bad".to_string()),
//            task_vec: TaskList { tasks: vec![] },
//        };
//        test_cli.task_vec.create_task().unwrap();
//        let error = test_cli.parse_arguments().unwrap_err().to_string();
//        assert_eq!(error, "invalid priority");
//    }

//    #[test] 
//    fn cli_task_reprioritize_duplicate_test() {
//        let mut test_cli = Cli {
//            pattern: Some("cp".to_string()),
//            index: Some("0".to_string()),
//            input: Some("Optional".to_string()),
//            task_vec: TaskList { tasks: vec![] },
//        };
//        test_cli.task_vec.create_task().unwrap();
//        let error = test_cli.parse_arguments().unwrap_err().to_string();
//        assert_eq!(error, "duplicate priority");
//    }

//    #[test] 
//    fn cli_task_rename_successful_test() {
//        let mut test_cli = Cli {
//            pattern: Some("rename".to_string()),
//            index: Some("0".to_string()),
//            input: Some("test rename function".to_string()),
//            task_vec: TaskList { tasks: vec![] },
//        };
//        test_cli.task_vec.create_task().unwrap();
//        let response = test_cli.parse_arguments().unwrap();
//        assert!(test_cli.task_vec.tasks[0].name == "test rename function".to_string());
//        assert_eq!(response, "Task Test Task renamed to test rename function");
//    }
//}
