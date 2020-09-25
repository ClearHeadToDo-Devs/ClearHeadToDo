use std::io::{self, Write};
use std::io::stdout;
use clear_head_todo::TaskList;
use std::path::Path;

pub struct CLI{
    pattern: String,
    index: Option<String>,
    input: Option<String>,
    task_vec: TaskList
}

impl CLI {
    pub fn parse_arguments(&mut self) {
        match &self.pattern as &str{
            "create_task" => self.task_vec
                .create_task(),
            "list_tasks" => self.task_vec
                .print_task_list(
                    io::stdout())
                        .unwrap(),
            "remove_task" => self.task_vec
                .remove_task(
                    self.index.as_ref()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(), 
                    io::stdout())
                        .expect("invalid index"),
            "complete_task" => self.task_vec.tasks[
                self.index.as_ref()
                .unwrap()
                .parse::<usize>()
                .unwrap()]
                    .mark_complete(),
            "change_priority" => self.task_vec.tasks[
                self.index.as_ref()
                .unwrap()
                .parse::<usize>()
                .unwrap()]
                    .change_priority(
                        &self.input.as_ref()
                        .unwrap()[..]),
            "rename_task" => self.task_vec.tasks[
                self.index.as_ref()
               .unwrap()
               .parse::<usize>()
               .unwrap()]
               .rename_task(
                    self.input.as_ref()
                   .unwrap()),
            _ => return
        }
    }
}
fn main() {

    println!("starting program");
    
    let mut main_cli: CLI = CLI{
        pattern : std::env::args().nth(1)
            .expect("no pattern given"), 
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
        let test_cli = CLI {
            pattern: "test_pattern".to_string(), 
            index: Some("test_index".to_string()),
            input: Some("test_input".to_string()),
            task_vec: TaskList{tasks: vec![]}, 

        };
        assert!(test_cli.pattern == "test_pattern".to_string());
    }
}
