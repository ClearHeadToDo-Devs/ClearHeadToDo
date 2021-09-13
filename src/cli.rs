use clear_head_todo::TaskList;
use std::error::Error;

extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub fn create_app() -> App<'static, 'static> {
    App::new("Clear Head Todo")
        .author("Darrion Burgess <darrionburgess@gmail.com>")
        .version("0.1.0")
        .about("can be used to manage every part of your productive life!")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("list_tasks").alias("lt"))
        .subcommand(SubCommand::with_name("create_task").alias("create"))
        .subcommand(
            SubCommand::with_name("complete_task")
                .alias("complete")
                .arg(Arg::with_name("index").required(true)),
        )
        .subcommand(
            SubCommand::with_name("remove_task")
                .alias("remove")
                .arg(Arg::with_name("index").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rename_task")
                .alias("rename")
                .arg(Arg::with_name("index").required(true))
                .arg(Arg::with_name("new_name").required(true).multiple(true)),
        )
        .subcommand(
            SubCommand::with_name("reprioritize")
                .alias("rp")
                .arg(Arg::with_name("index").required(true))
                .arg(Arg::with_name("new_priority").required(true)),
        )
}

#[derive(Debug, PartialEq)]
pub enum CliSubCommand {
    ListTasks,
    CreateTask,
    ToggleTaskCompletion(usize),
    RemoveTask(usize),
    RenameTask { index: usize, new_name: String },
    Reprioritize { index: usize, new_priority: String },
}

pub fn run(matches: ArgMatches<'_>) -> CliSubCommand {
    let outcome = match matches.subcommand_name() {
        Some("list_tasks") => CliSubCommand::ListTasks,
        Some("create_task") => CliSubCommand::CreateTask,
        Some("complete_task") => CliSubCommand::ToggleTaskCompletion(
            matches.parse_id_for_subcommand("complete_task".to_string()),
        ),
        Some("remove_task") => {
            CliSubCommand::RemoveTask(matches.parse_id_for_subcommand("remove_task".to_string()))
        }
        Some("rename_task") => CliSubCommand::RenameTask {
            index: matches.parse_id_for_subcommand("rename_task".to_string()),
            new_name: matches.parse_desired_name("rename_task".to_string()),
        },
        Some("reprioritize") => CliSubCommand::Reprioritize {
            index: matches.parse_id_for_subcommand("reprioritize".to_string()),
            new_priority: matches.parse_desired_priority("reprioritize".to_string()),
        },
        _ => unreachable!(),
    };
    return outcome;
}

pub fn run_subcommand(
    command: &CliSubCommand,
    task_list: &TaskList,
) -> Result<TaskList, Box<dyn Error>> {
    match command {
        CliSubCommand::ListTasks => {
            task_list.print_task_list()?;
            return Ok(task_list.clone());
        }
        CliSubCommand::CreateTask => {
            let updated_task_list = task_list.create_task();
            Ok(updated_task_list)
        }
        CliSubCommand::ToggleTaskCompletion(index) => {
            let updated_task_list = task_list.toggle_task_completion_status(*index)?;
            Ok(updated_task_list)
        }
        CliSubCommand::RemoveTask(index) => {
            let updated_task_list = task_list.remove_task(*index)?;
            Ok(updated_task_list)
        }
        CliSubCommand::RenameTask { index, new_name } => {
            let updated_task_list = task_list.rename_task(*index, new_name.to_string())?;
            Ok(updated_task_list)
        }
        CliSubCommand::Reprioritize {
            index,
            new_priority,
        } => {
            let updated_task_list =
                task_list.change_task_priority(*index, new_priority.to_string())?;
            Ok(updated_task_list)
        }
    }
}

pub fn create_end_user_message(
    updated_task_list: &TaskList,
    previous_task_list: &TaskList,
    subcommand: &CliSubCommand,
) -> String {
    match subcommand {
        CliSubCommand::CreateTask => {
            format!(
                "Created Task {}",
                updated_task_list.tasks[updated_task_list.tasks.len() - 1].name
            )
        }
        CliSubCommand::ToggleTaskCompletion(index) => {
            format!(
                "{} had its' completion status toggled to {}",
                updated_task_list.tasks[*index].name, updated_task_list.tasks[*index].completed
            )
        }
        CliSubCommand::RemoveTask(index) => {
            format!(
                "{} was removed from your Task List",
                previous_task_list.tasks[*index].name
            )
        }
        CliSubCommand::RenameTask { index, new_name } => {
            format!(
                "{} was changed from {}",
                updated_task_list.tasks[*index].name, new_name
            )
        }
        CliSubCommand::Reprioritize {
            index,
            new_priority,
        } => {
            format!(
                "{} was changed from a priority of: {}\n to a priority of: {}",
                updated_task_list.tasks[*index].name,
                previous_task_list.tasks[*index].priority,
                new_priority
            )
        }
        CliSubCommand::ListTasks => unreachable!(),
    }
}

trait SubcommandArgumentParser {
    fn parse_id_for_subcommand(&self, subcommand_name: String) -> usize;
    fn parse_desired_name(&self, subcommand_name: String) -> String;
    fn parse_desired_priority(&self, subcommand_name: String) -> String;
}

impl SubcommandArgumentParser for ArgMatches<'_> {
    fn parse_id_for_subcommand(&self, subcommand_name: String) -> usize {
        self.subcommand_matches(subcommand_name)
            .unwrap()
            .value_of("index")
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }

    fn parse_desired_name(&self, subcommand_name: String) -> String {
        self.subcommand_matches(subcommand_name)
            .unwrap()
            .values_of("new_name")
            .unwrap()
            .collect::<Vec<&str>>()
            .join(" ")
            .to_string()
    }

    fn parse_desired_priority(&self, subcommand_name: String) -> String {
        self.subcommand_matches(subcommand_name)
            .unwrap()
            .value_of("new_priority")
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::ErrorKind;
    use clear_head_todo::create_task_list;
    use clear_head_todo::PriEnum;
    use clear_head_todo::Task;
    use im::vector;
    use uuid::Uuid;

    #[test]
    fn cli_creation_name_test() {
        let app = create_app();

        assert_eq!(&app.p.meta.name, &"Clear Head Todo");
    }

    #[test]
    fn cli_creation_author_test() {
        let app = create_app();

        assert_eq!(
            &app.p.meta.author.unwrap(),
            &"Darrion Burgess <darrionburgess@gmail.com>"
        );
    }

    #[test]
    fn cli_creation_version_test() {
        let app = create_app();

        assert_eq!(app.p.meta.version.unwrap(), "0.1.0");
    }

    #[test]
    fn cli_creation_about_test() {
        let app = create_app();

        assert_eq!(
            app.p.meta.about.unwrap(),
            "can be used to manage every part of your productive life!"
        );
    }

    #[test]
    fn cli_creation_subcommand_or_help_test() {
        let app = create_app();

        let matches = app.get_matches_from_safe(&[""]);
        let error = matches.unwrap_err();

        assert_eq!(error.kind, ErrorKind::MissingArgumentOrSubcommand);
    }

    #[test]
    fn cli_list_task_successful_match_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "list_tasks"]);

        let result = run(test_matches);
        assert_eq!(result, CliSubCommand::ListTasks);
    }

    #[test]
    fn cli_list_task_successful_run_test() {
        let empty_task_list = create_task_list();
        let single_task_list = empty_task_list.add_nil_task();

        let result = run_subcommand(&CliSubCommand::ListTasks, &single_task_list);
        assert_eq!(
            result.unwrap(),
            TaskList {
                tasks: vector![clear_head_todo::Task {
                    id: Uuid::nil(),
                    ..Default::default()
                }]
            }
        );
    }

    #[test]
    fn cli_list_task_alias_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "lt"]);

        let result = run(test_matches);
        assert_eq!(result, CliSubCommand::ListTasks);
    }

    #[test]
    fn cli_list_task_failure_empty_list_test() {
        let empty_task_list = create_task_list();

        let error = run_subcommand(&CliSubCommand::ListTasks, &empty_task_list);
        assert_eq!(error.unwrap_err().to_string(), "list is empty");
    }

    #[test]
    fn cli_create_task_successful_parse_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "create_task"]);

        let result = run(test_matches);
        assert_eq!(result, CliSubCommand::CreateTask);
    }

    #[test]
    fn cli_create_task_successful_run_test() {
        let empty_task_list = create_task_list();
        let result = run_subcommand(&CliSubCommand::CreateTask, &empty_task_list).unwrap();

        assert_eq!(result.tasks[0].name, "Default Task".to_string());
        assert_eq!(result.tasks[0].priority, PriEnum::Optional);
        assert_eq!(result.tasks[0].completed, false);
    }

    #[test]
    fn cli_create_task_alias_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "create"]);

        let result = run(test_matches);
        assert_eq!(result, CliSubCommand::CreateTask);
    }

    #[test]
    fn cli_complete_task_successful_parse_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "complete_task", "0"]);

        let result = run(test_matches);
        assert_eq!(result, CliSubCommand::ToggleTaskCompletion(0));
    }

    #[test]
    fn cli_complete_task_successful_run_test() {
        let empty_task_list = create_task_list();
        let single_task_list = empty_task_list.add_nil_task();

        let result = run_subcommand(&CliSubCommand::ToggleTaskCompletion(0), &single_task_list);
        assert_eq!(
            result.unwrap(),
            TaskList {
                tasks: vector![Task {
                    completed: true,
                    id: Uuid::nil(),
                    ..Default::default()
                }]
            }
        );
    }

    #[test]
    fn cli_reopen_task_test() {
        let single_completed_task_list = TaskList {
            tasks: vector![Task {
                completed: true,
                id: Uuid::nil(),
                ..Default::default()
            }],
        };

        let updated_task_list = run_subcommand(
            &CliSubCommand::ToggleTaskCompletion(0),
            &single_completed_task_list,
        );
        assert_eq!(
            updated_task_list.unwrap(),
            TaskList {
                tasks: vector![Task {
                    id: Uuid::nil(),
                    ..Default::default()
                }]
            }
        )
    }

    #[test]
    fn cli_complete_task_alias_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "complete", "1"]);

        let result = run(test_matches);
        assert_eq!(result, CliSubCommand::ToggleTaskCompletion(1));
    }

    #[test]
    fn cli_complete_task_failing_invalid_id_test() {
        let empty_task_list = create_task_list();

        let error = run_subcommand(&CliSubCommand::ToggleTaskCompletion(1), &empty_task_list);
        assert_eq!(error.unwrap_err().to_string(), "No Task in that position");
    }

    #[test]
    fn cli_remove_task_successful_parse_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "remove_task", "1"]);

        let result = run(test_matches);
        assert_eq!(result, CliSubCommand::RemoveTask(1));
    }

    #[test]
    fn cli_remove_task_successful_run_test() {
        let empty_task_list = create_task_list();
        let single_task_list = empty_task_list.create_task();

        let result = run_subcommand(&CliSubCommand::RemoveTask(0), &single_task_list);
        assert_eq!(result.unwrap(), TaskList { tasks: vector![] });
    }

    #[test]
    fn cli_remove_task_alias_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "remove", "0"]);

        let result = run(test_matches);
        assert_eq!(result, CliSubCommand::RemoveTask(0));
    }

    #[test]
    fn failing_cli_remove_task_invalid_index_test() {
        let test_task_list = create_task_list();

        let error = run_subcommand(&CliSubCommand::RemoveTask(0), &test_task_list);
        assert_eq!(error.unwrap_err().to_string(), "No Task in that position");
    }

    #[test]
    fn successful_cli_rename_task_parse_test() {
        let app = create_app();
        let test_matches =
            app.get_matches_from(vec!["ClearHeadToDo", "rename_task", "0", "Test", "Rename"]);

        let result = run(test_matches);
        assert_eq!(
            result,
            CliSubCommand::RenameTask {
                index: 0,
                new_name: "Test Rename".to_string()
            }
        );
    }

    #[test]
    fn cli_rename_task_successful_run_test() {
        let empty_task_list = create_task_list();
        let single_task_list = empty_task_list.add_nil_task();

        let result = run_subcommand(
            &CliSubCommand::RenameTask {
                index: 0,
                new_name: "Test Rename".to_string(),
            },
            &single_task_list,
        );

        assert_eq!(
            result.unwrap(),
            TaskList {
                tasks: vector![Task {
                    name: "Test Rename".to_string(),
                    id: Uuid::nil(),
                    ..Default::default()
                }]
            }
        );
    }

    #[test]
    fn cli_rename_task_alias_test() {
        let app = create_app();
        let test_matches =
            app.get_matches_from(vec!["ClearHeadToDo", "rename", "0", "Test Rename"]);

        let result = run(test_matches);
        assert_eq!(
            result,
            CliSubCommand::RenameTask {
                index: 0,
                new_name: "Test Rename".to_string()
            }
        );
    }

    #[test]
    fn cli_rename_task_failing_invalid_id_test() {
        let test_task_list = create_task_list();

        let error = run_subcommand(
            &CliSubCommand::RenameTask {
                index: 0,
                new_name: "Test Rename".to_string(),
            },
            &test_task_list,
        );
        assert_eq!(error.unwrap_err().to_string(), "No Task in that position");
    }

    #[test]
    fn cli_change_priority_successful_parse_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "reprioritize", "1", "High"]);

        let result = run(test_matches);
        assert_eq!(
            result,
            CliSubCommand::Reprioritize {
                index: 1,
                new_priority: "High".to_string()
            }
        );
    }

    #[test]
    fn cli_change_priority_successful_run_test() {
        let empty_task_list = create_task_list();
        let single_task_list = empty_task_list.add_nil_task();

        let result = run_subcommand(
            &CliSubCommand::Reprioritize {
                index: 0,
                new_priority: "High".to_string(),
            },
            &single_task_list,
        );
        assert_eq!(
            result.unwrap(),
            TaskList {
                tasks: vector![Task {
                    priority: PriEnum::High,
                    id: Uuid::nil(),
                    ..Default::default()
                }]
            }
        );
    }

    #[test]
    fn cli_reprioritize_task_alias_test() {
        let app = create_app();
        let test_matches = app.get_matches_from(vec!["ClearHeadToDo", "rp", "1", "High"]);

        let result = run(test_matches);
        assert_eq!(
            result,
            CliSubCommand::Reprioritize {
                index: 1,
                new_priority: "High".to_string()
            }
        );
    }

    #[test]
    fn cli_reprioritize_failing_invalid_id_test() {
        let empty_task_list = create_task_list();

        let error = run_subcommand(
            &CliSubCommand::Reprioritize {
                index: 1,
                new_priority: "High".to_string(),
            },
            &empty_task_list,
        );
        assert_eq!(error.unwrap_err().to_string(), "No Task in that position");
    }
}
