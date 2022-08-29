mod arg_parser;
use arg_parser::create_app;
use arg_parser::ArgumentParsing;

use clear_head_todo_core::action::storage::load_action_from_csv;
use clear_head_todo_core::action::storage::load_csv_with_action_data;
use clear_head_todo_core::api_command::Command;
use clear_head_todo_core::Action;
use clear_head_todo_core::ActionListManipulation;
use im::vector;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let task_list = load_action_from_csv("actions.csv")?;

    let mut _updated_task_list: im::Vector<Action> = vector!();

    let app = create_app();
    let matches = app.get_matches();

    let subcommand = matches.parse_command()?;

    if subcommand == Command::List {
        let task_list_string_result = task_list.print_list();
        match task_list_string_result {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("{}", e),
        }
    } else {
        let updated_task_list = &subcommand.run_subcommand(&task_list)?;
        load_csv_with_action_data(&updated_task_list, "tasks.csv")?;
        println!(
            "{}",
            &subcommand.create_end_user_message(&task_list, &updated_task_list)
        );
    }

    Ok(())
}
