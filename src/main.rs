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

    let mut task_list = TaskList{ tasks: vec![], path: Path::new("./data/tasks.csv")};
    println!("starting program");
    
    task_list.load_tasks().unwrap();
    
    let main_cli = CLI{
        pattern : std::env::args().nth(1).expect("no pattern given"), 
        index: std::env::args().nth(2),
        input: std::env::args().nth(3)

    };

    match &main_cli.pattern as &str{
        "create_task" => task_list.create_task(),
        "list_tasks" => task_list.print_task_list(io::stdout()).unwrap(),
        "remove_task" => task_list.remove_task(
                        main_cli.index.unwrap().to_string().parse::<usize>().unwrap(), 
                        io::stdout()).expect("invalid index"),
        "complete_task" => task_list.tasks[
                            main_cli.index.unwrap().parse::<usize>().unwrap()]
                            .mark_complete(),
        "change_priority" => task_list.tasks[
                                main_cli.index.unwrap().parse::<usize>().unwrap()]
                                .change_priority(&main_cli.input.unwrap()[..]),
        "rename_task" => task_list.tasks[
                            main_cli.index.unwrap().parse::<usize>().unwrap()]
                            .rename_task(main_cli.input.unwrap()),
        _ => return
    }

    task_list.load_csv().unwrap();
    
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
