mod cli;

use cli::create_app;
use cli::run_subcommand;
use cli::run;
use clear_head_todo::create_task_list;
use clear_head_todo::TaskList;
use clear_head_todo::load_tasks_from_csv;
use clear_head_todo::load_csv;

fn main() {
    let mut task_list: TaskList = load_tasks_from_csv("tasks.csv").unwrap();

    let app = create_app();
    let matches = app.get_matches();

    let subcommand = run(matches);
    let result = run_subcommand(subcommand, task_list);

    match result {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("{}", e),
    }

    //load_csv(&task_list, "tasks.csv").unwrap();
}
