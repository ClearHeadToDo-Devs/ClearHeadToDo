use std::io::{self, Write};
use std::io::stdout;
use std::borrow::Borrow;
use clear_head_todo::TaskList;
use clear_head_todo::Task;

fn main() {
    let mut task_list = TaskList{ tasks: vec![]};
    println!("starting program");
    
    loop {
        let list = &mut task_list;
        print!("> ");
        io::stdout().flush().expect("failed to flush");
        
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("failed to read line");
        match parse_input(&inp, list) {
            Ok(op) => {
                println!("success!");
            },
            Err(err) => {
                println!("error: {}", err);
            }
        } //end 'match parse_input()'
        
    } //end 'loop'
} //end main

pub fn parse_input(inp: &str, list: &mut TaskList) -> Result<(), String> {
    match inp.to_ascii_lowercase().trim() {
        //"foo" => Ok("test_foo".to_string()),
        "create_task" => Ok(list.create_task()),
        "list_tasks" => Ok(list.print_task_list(&mut stdout()).unwrap()),
        _ => Err(format!("invalid input")),
    }
}
