use std::io::{self, Write};
use std::io::stdout;
use clear_head_todo::TaskList;
use std::path::Path;

struct CLI{
    pattern: String,
    index: Option<String>,
    input: Option<String>
}

fn main() {

    let mut task_list: TaskList = TaskList{tasks: vec![]};
    println!("starting program");
    
    task_list.load_tasks("tasks.csv").unwrap();
    
    let main_cli: CLI = CLI{
        pattern : std::env::args().nth(1).expect("no pattern given"), 
        index: std::env::args().nth(2),
        input: std::env::args().nth(3)

    };

    match &main_cli.pattern.to_ascii_lowercase() as &str{
        "create_task" | "create" | "ct" | "new_task" | "new" => task_list
            .create_task(),
        "list_tasks" | "lt" | "list" | "list_all" => task_list
            .print_task_list(
                io::stdout())
                .unwrap(),
        "remove_task" | "remove" | "rt" | "delete_task" | "delete" => task_list
            .remove_task(
                main_cli.index
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap(), 
                io::stdout())
                .expect("invalid index"),
        "complete_task" | "complete" | "mark_complete" => task_list.tasks[
            main_cli.index
            .unwrap()
            .parse::<usize>()
            .unwrap()]
            .mark_complete(),
        "change_priority" | "cp" | "new_priority" | "np" => task_list.tasks[
            main_cli.index
            .unwrap()
            .parse::<usize>()
            .unwrap()]
            .change_priority(
                    &main_cli.input.unwrap()[..]),
        "rename_task" | "rename" | "name" | "r" => task_list.tasks[
            main_cli.index
           .unwrap()
           .parse::<usize>()
           .unwrap()]
           .rename_task(
                main_cli.input
               .unwrap()),
        _ => return
    }

    task_list.load_csv("tasks.csv").unwrap();
    
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
