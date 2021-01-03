use clear_head_todo::PriEnum;
use clear_head_todo::TaskList;
use std::error::Error;


#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand, load_yaml, ArgMatches};

fn run(matches: ArgMatches,task_list: &mut TaskList)->Result<String, Box<dyn Error>>{
    let outcome = match matches.subcommand_name() {
        Some("list_tasks")=> task_list.print_task_list(std::io::stdout()),
        Some("create_task")=> task_list.create_task(),
        Some("complete_task")=> task_list.tasks[matches.subcommand_matches("complete_task").unwrap().value_of("index")
            .unwrap().parse::<usize>()?].mark_complete(),
        _ => Ok("Not a valid command, run --help to see the list of valid commands".to_string()),
    };
    return outcome
}

fn main() {
    let mut task_list: TaskList= TaskList{tasks: vec![]};

    task_list.load_tasks("tasks.csv").unwrap();
    let yaml = load_yaml!("config/cli_config.yaml");
    let matches = App::from(yaml).get_matches();
    let result = run(matches,&mut task_list);
    match result {
        Ok(s) => println!("{}",s),
        Err(e) => eprintln!("{}",e),
    }
    task_list.load_csv("tasks.csv").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn cli_creation_author_test() {
        let yaml = load_yaml!("config/cli_config.yaml");
        let m = App::from(yaml);
        assert_eq!(&m.p.meta.author.unwrap() , &"Darrion Burgess <darrionburgess@gmail.com>");
    }

    #[test]
    fn cli_creation_version_test() {
        let yaml = load_yaml!("config/cli_config.yaml");
        let m = App::from(yaml);
        assert_eq!(m.p.meta.version.unwrap() , "0.1.0");
    }

    #[test]
    fn cli_creation_about_test() {
        let yaml = load_yaml!("config/cli_config.yaml");
        let m = App::from(yaml);
        assert_eq!(m.p.meta.about.unwrap() , 
        "can be used to manage every part of your productive life!");
    }

    #[test]
    fn cli_list_task_successful_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "list_tasks"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "list_tasks");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "Successfully Printed {}");
    }

    #[test]
    fn cli_list_task_failure_empty_list_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(
            vec!["ClearHeadToDo", "list_tasks"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "list_tasks");

        let error = run(test_matches, &mut test_task_list);
        assert_eq!(error.unwrap_err().to_string(), "list is empty");
    }

    #[test]
    fn cli_create_task_successful_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "create_task"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "create_task");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "Created new task named Test Task");
        assert!(test_task_list.tasks.is_empty() == false)
    }

    #[test]
    fn cli_complete_task_successful_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "complete_task", "0"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "complete_task");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "completed Task: Test Task");
        assert!(test_task_list.tasks[0].completed == true)
    }

}
// pub struct Cli {
//     pub pattern: Option<String>,
//     pub index: Option<String>,
//     pub input: Option<String>,
//     pub task_vec: TaskList,
// }

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
