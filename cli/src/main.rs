mod arg_parser;
use crate::arg_parser::CommandParser;

use arg_parser::create_app;
use clear_head_todo_core::create_task_list;
use clear_head_todo_core::load_csv;
use clear_head_todo_core::load_tasks_from_csv;
use clear_head_todo_core::Command;
use clear_head_todo_core::TaskList;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let task_list: TaskList = load_tasks_from_csv("tasks.csv")?;

    let mut _updated_task_list = create_task_list();

    let app = create_app();
    let matches = app.get_matches();

    let subcommand = matches.parse_command();

    if subcommand == Command::ListTasks {
        let task_list_string_result = task_list.print_task_list();
        match task_list_string_result {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("{}", e),
        }
    } else {
        let updated_task_list = &subcommand.run_subcommand(&task_list)?;
        load_csv(&updated_task_list, "tasks.csv")?;
        println!(
            "{}",
            &subcommand.create_end_user_message(&task_list, &updated_task_list)
        );
    }

    Ok(())
}
