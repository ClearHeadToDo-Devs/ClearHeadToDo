use std::io::{self, Write};
use std::io::stdout;
use clear_head_todo::TaskList;
use std::path::Path;

fn main() {
    let mut task_list = TaskList{ tasks: vec![], path: Path::new("./data/testTasks.csv")};
    println!("starting program");
    
    loop {
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
    }
}
