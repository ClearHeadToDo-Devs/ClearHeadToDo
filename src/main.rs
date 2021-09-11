mod cli;
mod storage;
mod task;

use clear_head_todo::create_task_list;
use clear_head_todo::TaskList;
use cli::create_app;
use cli::run;
use cli::run_subcommand;
use cli::CliSubCommand;
use storage::load_csv;
use storage::load_tasks_from_csv;

fn main() {
    let task_list: TaskList = load_tasks_from_csv("tasks.csv").unwrap();

    let mut _updated_task_list = create_task_list();

    let app = create_app();
    let matches = app.get_matches();

    let subcommand = run(matches);

    if subcommand == CliSubCommand::ListTasks {
        let task_list_string_result = task_list.print_task_list();
        match task_list_string_result {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("{}", e),
        }
    } else {
        let _updated_task_list = run_subcommand(subcommand, &task_list);
    }

    load_csv(&_updated_task_list, "tasks.csv").unwrap();
}
