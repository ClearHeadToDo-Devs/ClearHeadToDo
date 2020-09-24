use std::io::{self, Write};
use std::io::stdout;
use clear_head_todo::TaskList;
use std::path::Path;

struct CLI{
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
                    .to_string()
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
                        &self.input.as_ref().unwrap()[..]),
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
        pattern : std::env::args().nth(1).expect("no pattern given"), 
        index: std::env::args().nth(2),
        input: std::env::args().nth(3),
        task_vec: TaskList{tasks: vec![]}
    };

    main_cli.task_vec.load_tasks("tasks.csv").unwrap();
    
    main_cli.parse_arguments();

    main_cli.task_vec.load_csv("tasks.csv").unwrap();
    
/*    loop {
        let list = &mut task_list;
        print!("> ");
        io::stdout().flush().expect("failed to flush");
        
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("failed to read line");
        let mut words = inp.trim().split_whitespace();
        let command = words.next().unwrap();
        let index = words.next().unwrap().parse::<usize>().unwrap();
        let arg = words.next().unwrap();
  
        
        match parse_input(command, index, &arg, list) {
            Ok(op) => {
                println!("success!");
            },
            Err(err) => {
                println!("error: {}", err);
            }
        } //end 'match parse_input()'
        
    } //end 'loop'
} //end main

pub fn parse_input(inp: &str, index: usize, arg: &str, list: &mut TaskList) -> Result<(), String> {
    match inp.to_ascii_lowercase().trim() {
        //"foo" => Ok("test_foo".to_string()),
        "create_task" => Ok(list.create_task()),
        "list_tasks" => Ok(list.print_task_list(&mut stdout()).unwrap()),
        "change_priority" => Ok(list.tasks[index].change_priority(arg)),
        _ => Err(format!("invalid input")),
    }*/
}
