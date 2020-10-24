use std::io::{self, Write};
use std::io::stdout;
use clear_head_todo::TaskList;
use clear_head_todo::PriEnum;
use std::path::Path;

pub struct CLI{
    pub pattern: Option<String>,
    pub index: Option<String>,
    pub input: Option<String>,
    pub task_vec: TaskList
}

impl CLI {
        pub fn parse_arguments(&mut self) -> Result<String, String> {
            match self.pattern.as_ref().unwrap_or(
                &"no command given".to_string()) as &str{
            "create_task" | "create" | "ct" | "new_task" | "new" =>
                self.task_vec
                .create_task(),
            "list_tasks" | "lt" | "list" | "list_all" =>
                self.task_vec.print_task_list(std::io::stdout()).unwrap_or(()),
            "remove_task" | "remove" | "rt" | "delete_task" | "delete" =>
                self.task_vec
                .remove_task(
                    self.index.as_ref()
                    .unwrap()
                    .to_string()
                    .parse::<usize>()
                    .unwrap(), 
                    io::stdout())
                    .expect("invalid index"),
            "complete_task" | "complete" | "mark_complete" =>
                self.task_vec.tasks[
                self.index.as_ref()
                .unwrap()
                .parse::<usize>()
                .unwrap()]
                .mark_complete(),
            "change_priority" | "cp" | "new_priority" | "np" =>
                self.task_vec.tasks[
                self.index.as_ref()
                .unwrap()
                .parse::<usize>()
                .unwrap()]
                .change_priority(
                        &self.input.as_ref().unwrap()[..]),
            "rename_task" | "rename" | "name" | "r" =>
                self.task_vec.tasks[
                self.index.as_ref()
               .unwrap()
               .parse::<usize>()
               .unwrap()]
               .rename_task(
                    self.input.as_ref()
                   .unwrap()),
            _ => return
            };
    }
    pub fn cli_list_tasks(&self, mut writer: impl std::io::Write){
        self.task_vec.print_task_list(writer).unwrap_or(());
    }
}
fn main() {

    println!("starting program");
    
    let mut main_cli: CLI = CLI{
        pattern : std::env::args().nth(1),
        index: std::env::args().nth(2),
        input: std::env::args().nth(3),
        task_vec: TaskList{
            tasks: vec![]
        }
    };

    main_cli.task_vec.load_tasks("tasks.csv").unwrap();
    
    main_cli.parse_arguments();

    main_cli.task_vec.load_csv("tasks.csv").unwrap();
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_creation_test () {
        let mut test_cli = CLI {
            pattern: None, 
            index: None,
            input: None,
            task_vec: TaskList{tasks: vec![]}, 

        };
        assert!(test_cli.pattern == None);
        assert!(test_cli.index == None);
        assert!(test_cli.input == None);
        assert!(test_cli.task_vec.tasks.len() == 0);
        test_cli.parse_arguments();
    }

    #[test]
    fn cli_task_creation_test () {
        let mut test_cli = CLI {
            pattern: Some("create_task".to_string()), 
            index: None,
            input: None,
            task_vec: TaskList{tasks: vec![]}, 

        };
        test_cli.parse_arguments();
        assert!(test_cli.task_vec.tasks.len() == 1);
        assert!(test_cli.task_vec.tasks[0].name == "Test Task");
        assert!(test_cli.task_vec.tasks[0].completed == false);
        assert!(test_cli.task_vec.tasks[0].priority == PriEnum::Optional);
    }

    #[test]
    fn cli_task_list_test () {
        //let mut good_result = Vec::new();
        let mut test_cli = CLI {
            pattern: Some("list_tasks".to_string()), 
            index: None,
            input: None,
            task_vec: TaskList{tasks: vec![]}, 

        };

        test_cli.parse_arguments();
    }
}
