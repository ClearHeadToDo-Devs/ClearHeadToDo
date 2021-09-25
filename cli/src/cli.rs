use clear_head_todo_core::TaskList;
use std::error::Error;

extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

#[derive(Debug, PartialEq)]
pub enum CliSubCommand {
    ListTasks,
    CreateTask,
    ToggleTaskCompletion(usize),
    RemoveTask(usize),
    RenameTask { index: usize, new_name: String },
    Reprioritize { index: usize, new_priority: String },
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
    subcommand: &CliSubCommand,
    previous_task_list: &TaskList,
    updated_task_list: &TaskList,
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::ErrorKind;
    use clear_head_todo_core::create_task_list;
    use clear_head_todo_core::PriEnum;
    use clear_head_todo_core::Task;
    use im::vector;
    use uuid::Uuid;

    #[test]
    fn cli_list_task_failure_empty_list_test() {
        let empty_task_list = create_task_list();

        let error = run_subcommand(&CliSubCommand::ListTasks, &empty_task_list);
        assert_eq!(error.unwrap_err().to_string(), "list is empty");
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
    fn cli_complete_task_failing_invalid_id_test() {
        let empty_task_list = create_task_list();

        let error = run_subcommand(&CliSubCommand::ToggleTaskCompletion(1), &empty_task_list);
        assert_eq!(error.unwrap_err().to_string(), "No Task at Given Index");
    }

    #[test]
    fn cli_remove_task_successful_run_test() {
        let empty_task_list = create_task_list();
        let single_task_list = empty_task_list.create_task();

        let result = run_subcommand(&CliSubCommand::RemoveTask(0), &single_task_list);
        assert_eq!(result.unwrap(), TaskList { tasks: vector![] });
    }

    #[test]
    fn failing_cli_remove_task_invalid_index_test() {
        let test_task_list = create_task_list();

        let error = run_subcommand(&CliSubCommand::RemoveTask(0), &test_task_list);
        assert_eq!(error.unwrap_err().to_string(), "No Task at Given Index");
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
    fn cli_rename_task_failing_invalid_id_test() {
        let test_task_list = create_task_list();

        let error = run_subcommand(
            &CliSubCommand::RenameTask {
                index: 0,
                new_name: "Test Rename".to_string(),
            },
            &test_task_list,
        );
        assert_eq!(error.unwrap_err().to_string(), "No Task at Given Index");
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
    fn cli_reprioritize_failing_invalid_id_test() {
        let empty_task_list = create_task_list();

        let error = run_subcommand(
            &CliSubCommand::Reprioritize {
                index: 1,
                new_priority: "High".to_string(),
            },
            &empty_task_list,
        );
        assert_eq!(error.unwrap_err().to_string(), "No Task at Given Index");
    }
}
