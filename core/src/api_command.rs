use crate::item::Action;
use crate::ActionListManipulation;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Command {
    ListTasks,
    CreateTask(Option<String>),
    ToggleTaskCompletion(usize),
    RemoveTask(usize),
    RenameTask { index: usize, new_name: String },
    Reprioritize { index: usize, new_priority: String },
}

impl Command {
    pub fn run_subcommand(
        &self,
        task_list: &im::Vector<Action>,
    ) -> Result<im::Vector<Action>, Box<dyn Error>> {
        match self {
            Command::ListTasks => {
                task_list.print_list()?;
                return Ok(task_list.clone());
            }
            Command::CreateTask(name) => {
                let updated_task_list = task_list.create_new();
                if let Some(name) = name {
                    return updated_task_list
                        .rename(updated_task_list.len() - 1, name.to_string());
                }
                Ok(updated_task_list)
            }
            Command::ToggleTaskCompletion(index) => {
                let updated_task_list = task_list.toggle_completion_status(*index)?;
                Ok(updated_task_list)
            }
            Command::RemoveTask(index) => {
                let updated_task_list = task_list.remove(*index)?;
                Ok(updated_task_list)
            }
            Command::RenameTask { index, new_name } => {
                let updated_task_list = task_list.rename(*index, new_name.to_string())?;
                Ok(updated_task_list)
            }
            Command::Reprioritize {
                index,
                new_priority,
            } => {
                let updated_task_list =
                    task_list.change_priority(*index, new_priority.to_string())?;
                Ok(updated_task_list)
            }
        }
    }

    pub fn create_end_user_message(
        &self,
        previous_task_list: &im::Vector<Action>,
        updated_task_list: &im::Vector<Action>,
    ) -> String {
        match self {
            Command::CreateTask(_name) => {
                format!(
                    "Created Task {}",
                    updated_task_list[updated_task_list.len() - 1].name
                )
            }
            Command::ToggleTaskCompletion(index) => {
                format!(
                    "{} had its' completion status toggled to {}",
                    updated_task_list[*index].name, updated_task_list[*index].completed
                )
            }
            Command::RemoveTask(index) => {
                format!(
                    "{} was removed from your Task List",
                    previous_task_list[*index].name
                )
            }
            Command::RenameTask { index, new_name } => {
                format!(
                    "{} was changed from {}",
                    new_name, previous_task_list[*index].name
                )
            }
            Command::Reprioritize {
                index,
                new_priority,
            } => {
                format!(
                    "{} was changed from a priority of: {}\n to a priority of: {}",
                    updated_task_list[*index].name,
                    previous_task_list[*index].priority,
                    new_priority
                )
            }
            Command::ListTasks => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::add_nil_action;
    use crate::Priority;
    use crate::Action;
    use im::vector;
    use uuid::Uuid;

    #[test]
    fn list_task_failure_empty_list() {
        let empty_task_list = vector!();

        let error = Command::ListTasks.run_subcommand(&empty_task_list);
        assert_eq!(error.unwrap_err().to_string(), "list is empty");
    }

    #[test]
    fn create_task_successful_run() {
        let empty_task_list = vector!();
        let result = Command::CreateTask(None)
            .run_subcommand(&empty_task_list)
            .unwrap();

        assert_eq!(result[0].name, "Default Action".to_string());
        assert_eq!(result[0].priority, Priority::Optional);
        assert_eq!(result[0].completed, false);
    }

    #[test]
    fn generate_create_task_success_message() {
        let empty_task_list = vector!();
        let single_task_list = empty_task_list.create_new();

        let message =
            Command::CreateTask(None).create_end_user_message(&empty_task_list, &single_task_list);
        assert_eq!(message, "Created Task Default Action");
    }

    #[test]
    fn complete_task_successful_run() {
        let single_task_list = add_nil_action(im::vector!());

        let result = Command::ToggleTaskCompletion(0).run_subcommand(&single_task_list);

        assert_eq!(
            result.unwrap(),
            vector![Action {
                completed: true,
                id: Uuid::nil(),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn reopen_task_successful_run() {
        let single_completed_task_list = vector![Action {
            completed: true,
            id: Uuid::nil(),
            ..Default::default()
        }];

        let updated_task_list =
            Command::ToggleTaskCompletion(0).run_subcommand(&single_completed_task_list);
        assert_eq!(
            updated_task_list.unwrap(),
            vector![Action {
                id: Uuid::nil(),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn generate_complete_task_message() {
        let single_task_list = vector!().create_new();
        let updated_task_list = single_task_list.toggle_completion_status(0).unwrap();

        let message = Command::ToggleTaskCompletion(0)
            .create_end_user_message(&single_task_list, &updated_task_list);

        assert_eq!(
            message,
            "Default Action had its\' completion status toggled to true"
        );
    }

    #[test]
    fn complete_task_failing_invalid_id() {
        let empty_task_list = vector!();

        let error = Command::ToggleTaskCompletion(1).run_subcommand(&empty_task_list);
        assert_eq!(error.unwrap_err().to_string(), "No Action at Index 1");
    }

    #[test]
    fn cli_remove_task_successful_run_test() {
        let empty_task_list = vector!();
        let single_task_list = empty_task_list.create_new();

        let result = Command::RemoveTask(0).run_subcommand(&single_task_list);
        assert_eq!(result.unwrap(), vector!());
    }

    #[test]
    fn generate_remove_task_message() {
        let single_task_list = vector!().create_new();
        let updated_task_list = single_task_list.remove(0).unwrap();

        let message =
            Command::RemoveTask(0).create_end_user_message(&single_task_list, &updated_task_list);

        assert_eq!(message, "Default Action was removed from your Task List");
    }

    #[test]
    fn failing_cli_remove_task_invalid_index_test() {
        let test_task_list = vector!();

        let error = Command::RemoveTask(0).run_subcommand(&test_task_list);
        assert_eq!(error.unwrap_err().to_string(), "No Action at Index 0");
    }

    #[test]
    fn cli_rename_task_successful_run_test() {
        let empty_task_list = vector!();
        let single_task_list = add_nil_action(empty_task_list);

        let result = Command::RenameTask {
            index: 0,
            new_name: "Test Rename".to_string(),
        }
        .run_subcommand(&single_task_list);

        assert_eq!(
            result.unwrap(),
            vector![Action {
                name: "Test Rename".to_string(),
                id: Uuid::nil(),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn generate_rename_task_message() {
        let single_task_list = vector!().create_new();
        let updated_task_list = single_task_list
            .rename(0, "New Name".to_string())
            .unwrap();

        let message = Command::RenameTask {
            index: 0,
            new_name: "New Name".to_string(),
        }
        .create_end_user_message(&single_task_list, &updated_task_list);

        assert_eq!(message, "New Name was changed from Default Action");
    }

    #[test]
    fn cli_rename_task_failing_invalid_id_test() {
        let test_task_list = vector!();

        let error = Command::RenameTask {
            index: 0,
            new_name: "Test Rename".to_string(),
        }
        .run_subcommand(&test_task_list);
        assert_eq!(error.unwrap_err().to_string(), "No Action at Index 0");
    }

    #[test]
    fn cli_change_priority_successful_run_test() {
        let empty_task_list = vector!();
        let single_task_list = add_nil_action(empty_task_list);

        let result = Command::Reprioritize {
            index: 0,
            new_priority: "High".to_string(),
        }
        .run_subcommand(&single_task_list);
        assert_eq!(
            result.as_ref().unwrap(),
            &vector![Action {
                priority: Priority::High,
                id: Uuid::nil(),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn generate_reprioritize_task_message() {
        let single_task_list = vector!().create_new();
        let updated_task_list = single_task_list
            .change_priority(0, "low".to_string())
            .unwrap();

        let message = Command::Reprioritize {
            index: 0,
            new_priority: "low".to_string(),
        }
        .create_end_user_message(&single_task_list, &updated_task_list);

        assert_eq!(
            message,
            "Default Action was changed from a priority of: Optional\n to a priority of: low"
        );
    }

    #[test]
    fn cli_reprioritize_failing_invalid_id_test() {
        let empty_task_list = vector!();

        let error = Command::Reprioritize {
            index: 1,
            new_priority: "High".to_string(),
        }
        .run_subcommand(&empty_task_list);
        assert_eq!(error.unwrap_err().to_string(), "No Action at Index 1");
    }
}
