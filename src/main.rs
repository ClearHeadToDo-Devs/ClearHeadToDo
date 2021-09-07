mod cli;
mod storage;
mod task;

use clear_head_todo::TaskList;
use cli::create_app;
use cli::run;
use cli::run_subcommand;
use storage::load_csv;
use storage::load_tasks_from_csv;

fn main() {
    let task_list: TaskList = load_tasks_from_csv("tasks.csv").unwrap();

    let app = create_app();
    let matches = app.get_matches();

    let subcommand = run(matches);
    let result = run_subcommand(subcommand, &task_list);

    match result {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("{}", e),
    }

    //load_csv(&task_list, "tasks.csv").unwrap();
}
