use clear_head_todo::PriEnum;
use clear_head_todo::TaskList;
use std::error::Error;


#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand, load_yaml, ArgMatches};

enum CliSubCommand {
    ListTasks,
    CreateTask,
    CompleteTask (usize),
    RemoveTask (usize),
    RenameTask {index: usize, new_name: String},
    Reprioritize {index: usize, new_priority: String},
}

fn run_subcommand(command: CliSubCommand, task_list: &mut TaskList) -> Result<String, Box<dyn Error>>{
   match command {
        CliSubCommand::ListTasks => task_list.print_task_list(std::io::stdout()),
        CliSubCommand::CreateTask => task_list.create_task(),
        CliSubCommand::CompleteTask(i) => task_list.tasks.get_mut(i)
            .ok_or("Out of Bounds Index")?.mark_complete(),
        CliSubCommand::RemoveTask(i) => task_list.remove_task(i),
        CliSubCommand::RenameTask{index,new_name} => task_list.tasks.get_mut(index)
            .ok_or("Out of Bounds Index")?.rename_task(&new_name),
        CliSubCommand::Reprioritize{index,new_priority} => task_list.tasks.get_mut(index)
            .ok_or("Out of Bounds Index")?.change_priority(&new_priority[..]),
   }
}

fn run(matches: ArgMatches,task_list: &mut TaskList)->Result<String, Box<dyn Error>>{
    let outcome = match matches.subcommand_name() {
        Some("list_tasks")=> run_subcommand(CliSubCommand::ListTasks, task_list),
        Some("create_task")=> run_subcommand(CliSubCommand::CreateTask, task_list),
        Some("complete_task")=> run_subcommand(CliSubCommand::CompleteTask(
                matches.subcommand_matches("complete_task").unwrap()
            .value_of("index").unwrap().parse::<usize>()?),task_list),
        Some("remove_task")=> run_subcommand(CliSubCommand::RemoveTask(matches.subcommand_matches("remove_task").unwrap()
            .value_of("index").unwrap().parse::<usize>()?), task_list),
        Some("rename_task")=> run_subcommand(CliSubCommand::RenameTask{
            index: matches.subcommand_matches("rename_task").unwrap()
            .value_of("index").unwrap().parse::<usize>()?, 
            new_name: matches.subcommand_matches("rename_task").unwrap()
             .values_of("new_name").unwrap().collect::<Vec<&str>>().join(" ").to_string()
                }, task_list),
        Some("reprioritize")=> run_subcommand(CliSubCommand::Reprioritize{
            index: matches.subcommand_matches("reprioritize").unwrap()
            .value_of("index").unwrap().parse::<usize>()?,
            new_priority: matches.subcommand_matches("reprioritize").unwrap()
                .value_of("new_priority").unwrap().to_string()}
                , task_list),
        _ => Ok("Not a valid command, run --help to see the list of valid commands".to_string()),
    };
    return outcome
}

fn main() {
    let mut task_list: TaskList= TaskList{tasks: vec![]};
    task_list.load_tasks("tasks.csv").unwrap();

    let yaml = load_yaml!("config/cli_config.yaml");
    let matches = App::from(yaml).get_matches();

    let result = run(matches,&mut task_list);
    match result {
        Ok(s) => println!("{}",s),
        Err(e) => eprintln!("{}",e),
    }

    task_list.load_csv("tasks.csv").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn cli_creation_author_test() {
        let yaml = load_yaml!("config/cli_config.yaml");
        let m = App::from(yaml);
        assert_eq!(&m.p.meta.author.unwrap() , &"Darrion Burgess <darrionburgess@gmail.com>");
    }

    #[test]
    fn cli_creation_version_test() {
        let yaml = load_yaml!("config/cli_config.yaml");
        let m = App::from(yaml);
        assert_eq!(m.p.meta.version.unwrap() , "0.1.0");
    }

    #[test]
    fn cli_creation_about_test() {
        let yaml = load_yaml!("config/cli_config.yaml");
        let m = App::from(yaml);
        assert_eq!(m.p.meta.about.unwrap() , 
        "can be used to manage every part of your productive life!");
    }

    #[test]
    fn cli_list_task_successful_match_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "list_tasks"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "list_tasks");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "End of List");
    }

    #[test]
    fn cli_list_task_successful_run_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "list_tasks"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "list_tasks");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "End of List");
    }

    #[test]
    fn cli_list_task_alias_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "lt"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "list_tasks");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "End of List");
    }

    #[test]
    fn cli_list_task_failure_empty_list_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(
            vec!["ClearHeadToDo", "list_tasks"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "list_tasks");

        let error = run(test_matches, &mut test_task_list);
        assert_eq!(error.unwrap_err().to_string(), "list is empty");
    }

    #[test]
    fn cli_create_task_successful_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "create_task"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "create_task");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "Created new task named Test Task");
        assert!(test_task_list.tasks.is_empty() == false)
    }

    #[test]
    fn cli_create_task_alias_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "create"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "create_task");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "Created new task named Test Task");
        assert!(test_task_list.tasks.is_empty() == false)
    }

    #[test]
    fn cli_complete_task_successful_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "complete_task", "0"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "complete_task");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "completed Task: Test Task");
        assert!(test_task_list.tasks[0].completed == true);
    }

    #[test]
    fn cli_complete_task_alias_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "complete", "0"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "complete_task");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "completed Task: Test Task");
        assert!(test_task_list.tasks[0].completed == true);
    }

    #[test]
    fn cli_complete_task_failing_invalid_index_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "complete_task", "0"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "complete_task");

        let error = run(test_matches, &mut test_task_list);
        assert_eq!(error.unwrap_err().to_string(), "Out of Bounds Index");
    }

    #[test]
    fn cli_complete_task_failing_already_complete_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        test_task_list.tasks[0].mark_complete().unwrap();

        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "complete_task", "0"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "complete_task");

        let error = run(test_matches, &mut test_task_list);
        assert_eq!(error.unwrap_err().to_string(), "Task is already completed");
        assert!(test_task_list.tasks[0].completed == true);
    }

    #[test]
    fn cli_remove_task_successful_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "remove_task", "0"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "remove_task");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "Successfully Removed Task Test Task");
        assert!(test_task_list.tasks.is_empty());
    }

    #[test]
    fn cli_remove_task_alias_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "delete", "0"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "remove_task");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "Successfully Removed Task Test Task");
        assert!(test_task_list.tasks.is_empty());
    }

    #[test]
    fn cli_remove_task_failing_invalid_index_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "remove_task", "0"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "remove_task");

        let error = run(test_matches, &mut test_task_list);
        assert_eq!(error.unwrap_err().to_string(), "Invalid Index for Deletion");
    }

    #[test]
    fn cli_rename_task_successful_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "rename_task", "0", "Test", "Rename"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "rename_task");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "Task Test Task renamed to Test Rename");
        assert!(test_task_list.tasks[0].name == "Test Rename");
    }

    #[test]
    fn cli_rename_task_alias_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "rename", "0", "Test Rename"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "rename_task");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "Task Test Task renamed to Test Rename");
        assert!(test_task_list.tasks[0].name == "Test Rename");
    }

    #[test]
    fn cli_rename_task_failing_invalid_index_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "rename_task", "0", "Test Rename"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "rename_task");

        let error = run(test_matches, &mut test_task_list);
        assert_eq!(error.unwrap_err().to_string(), "Out of Bounds Index");
    }

    #[test]
    fn cli_change_priority_successful_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "reprioritize", "0", "High"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "reprioritize");

        let result = run(test_matches, &mut test_task_list);
        assert_eq!(result.unwrap(), "changed Task: Test Task priority changed to High");
        assert!(test_task_list.tasks[0].priority == PriEnum::High);
    }

    #[test]
    fn cli_reprioritize_failing_invalid_index_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "reprioritize", "0", "High"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "reprioritize");

        let error = run(test_matches, &mut test_task_list);
        assert_eq!(error.unwrap_err().to_string(), "Out of Bounds Index");
    }

    #[test]
    fn cli_reprioritize_duplicate_failing_test() {
        let mut test_task_list = TaskList{tasks: vec![]};
        test_task_list.create_task().unwrap();
        let yaml = load_yaml!("config/cli_config.yaml");
        let test_matches = App::from(yaml).get_matches_from(vec!["ClearHeadToDo", "reprioritize", "0", "Optional"]);
        assert_eq!(test_matches.subcommand_name().unwrap(), "reprioritize");

        let error = run(test_matches, &mut test_task_list);
        assert_eq!(error.unwrap_err().to_string(), "duplicate priority");
    }
}
