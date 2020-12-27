use clear_head_todo::PriEnum;
use clear_head_todo::TaskList;
use std::error::Error;
use std::io::stdout;
use std::io::{self, Write};
use std::path::Path;

pub struct Cli {
    pub pattern: Option<String>,
    pub index: Option<usize>,
    pub input: Option<String>,
    pub task_vec: TaskList,
}

impl Cli {
    pub fn parse_arguments(&mut self) -> Result<String, Box<dyn Error>> {
        match self.pattern.as_ref().unwrap() as &str {
            "create_task" | "create" | "ct" | "new_task" | "new" => self.task_vec.create_task(),
            "list_tasks" | "lt" | "list" | "list_all" => {
                self.task_vec.print_task_list(std::io::stdout())
            }
            "remove_task" | "remove" | "rt" | "delete_task" | "delete" => {
                self.task_vec.remove_task(self.index.unwrap())
            }
            "complete_task" | "complete" | "mark_complete" => {
                self.task_vec.tasks[self.index.unwrap()].mark_complete()
            }
            "change_priority" | "cp" | "new_priority" | "np" => self.task_vec.tasks
                [self.index.unwrap()]
            .change_priority(&self.input.as_ref().unwrap()[..]),
            "rename_task" | "rename" | "name" | "r" => {
                self.task_vec.tasks[self.index.unwrap()].rename_task(self.input.as_ref().unwrap())
            }
            _ => return Ok("Try putting in a command to see what we can do!".to_string()),
        }
    }
}

fn main() {
    println!("starting program");

    let mut main_cli: Cli = Cli {
        pattern: std::env::args().nth(1),
        index: Some(
            std::env::args()
                .nth(2)
                .as_ref()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap(),
        ),
        input: std::env::args().nth(3),
        task_vec: TaskList { tasks: vec![] },
    };

    main_cli.task_vec.load_tasks("tasks.csv").unwrap();

    match main_cli.parse_arguments() {
        Ok(output) => println!("{}", output),
        Err(error) => println!("{}", error),
    };

    main_cli.task_vec.load_csv("tasks.csv").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_creation_test() {
        let mut test_cli = Cli {
            pattern: None,
            index: None,
            input: None,
            task_vec: TaskList { tasks: vec![] },
        };
        assert!(test_cli.pattern == None);
        assert!(test_cli.index == None);
        assert!(test_cli.input == None);
        assert!(test_cli.task_vec.tasks.len() == 0);
    }

    #[test]
    fn cli_task_creation_test() {
        let mut test_cli = Cli {
            pattern: Some("create_task".to_string()),
            index: None,
            input: None,
            task_vec: TaskList { tasks: vec![] },
        };
        test_cli.parse_arguments().unwrap();
        assert!(test_cli.task_vec.tasks.len() == 1);
        assert!(test_cli.task_vec.tasks[0].name == "Test Task");
        assert!(test_cli.task_vec.tasks[0].completed == false);
        assert!(test_cli.task_vec.tasks[0].priority == PriEnum::Optional);
    }

    #[test]
    fn cli_task_list_test() {
        //let mut good_result = Vec::new();
        let mut test_cli = Cli {
            pattern: Some("list_tasks".to_string()),
            index: None,
            input: None,
            task_vec: TaskList { tasks: vec![] },
        };
        test_cli.task_vec.create_task().unwrap();
        let response: String = test_cli.parse_arguments().unwrap();
        assert_eq!(response, "Successfully Printed {}");
    }
}
