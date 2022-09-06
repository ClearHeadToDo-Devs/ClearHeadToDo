pub mod arg_parser;
use arg_parser::create_app;
use arg_parser::ArgumentParsing;

pub mod command_runner;
use command_runner::Command;

use clear_head_todo_core::ClearHeadApp;
use clear_head_todo_core::JSONStorage;

use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let clear_head_app = ClearHeadApp::read_from_json(Path::new("cli/data/app.json"))?;

    let argument_parser = create_app();
    let matches = argument_parser.get_matches();
    let subcommand = matches.parse_command()?;

    if subcommand == Command::List {
        println!("{}", clear_head_app.get_list());
        return Ok(())
    } else if subcommand == Command::ExtendedList{
        match clear_head_app.get_extended_list() {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("{}", e),
        }
    }
    else {
        let updated_task_list = subcommand.run_subcommand(&clear_head_app)?;
        updated_task_list.write_to_json(Path::new("cli/data/app.json"),true)?;
        println!(
            "{}",
            &subcommand.create_end_user_message(&clear_head_app, &updated_task_list)
        );
    }

    Ok(())
}
