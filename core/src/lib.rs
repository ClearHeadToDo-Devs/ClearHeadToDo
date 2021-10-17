extern crate task;
pub use task::*;

pub mod helper;
pub use helper::*;

pub mod storage;
pub use storage::*;

pub mod command;
pub use command::*;

use im::vector;
use std::error::Error;
use std::io::{Error as OtherError, ErrorKind};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct TaskList {
    pub tasks: im::Vector<Task>,
}

pub trait TaskListManipulation {
    type Child;
    fn create_task(&self) -> Self;
    fn print_task_list(&self) -> Result<String, Box<dyn Error>>;
    fn remove_task(&self, index: usize) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn rename_task(&self, index: usize, new_name: String) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn toggle_task_completion_status(&self, index: usize) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn change_task_priority(
        &self,
        index: usize,
        new_priority: String,
    ) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn select_task_by_id(&self, id: Uuid) -> Result<Self::Child, Box<dyn Error>>;
}

impl TaskListManipulation for TaskList {
    type Child = Task;
    fn create_task(&self) -> Self {
        let mut new_list = self.clone();
        new_list.tasks.push_back(Task::create_default_task());
        return new_list;
    }

    fn print_task_list(&self) -> Result<String, Box<dyn Error>> {
        let mut task_list_string = "name,priority,completed,ID\n".to_string();

        if self.tasks.is_empty() == true {
            return Err(Box::new(OtherError::new(ErrorKind::Other, "list is empty")));
        } else {
            for task in &self.tasks {
                task_list_string.push_str(&task.export_fields_as_string());
            }
        }

        Ok(task_list_string.to_owned())
    }

    fn remove_task(&self, index: usize) -> Result<TaskList, Box<dyn Error>> {
        match self.tasks.iter().nth(index) {
            Some(_task_ref) => {
                let (mut left_side, mut right_side) = self.tasks.clone().split_at(index);
                right_side.pop_front().unwrap();
                left_side.append(right_side);
                Ok(TaskList { tasks: left_side })
            }
            None => Err(Box::new(OtherError::new(
                ErrorKind::Other,
                "No Task at Given Index",
            ))),
        }
    }

    fn rename_task(&self, index: usize, new_name: String) -> Result<TaskList, Box<dyn Error>> {
        match self.tasks.iter().nth(index) {
            Some(task_ref) => Ok(TaskList {
                tasks: self.tasks.update(index, task_ref.rename(&new_name)),
            }),
            None => Err(Box::new(OtherError::new(
                ErrorKind::Other,
                "No Task at Given Index",
            ))),
        }
    }

    fn toggle_task_completion_status(&self, index: usize) -> Result<TaskList, Box<dyn Error>> {
        match self.tasks.iter().nth(index) {
            Some(task_ref) => Ok(TaskList {
                tasks: self
                    .tasks
                    .update(index, task_ref.clone().toggle_completion_status()),
            }),
            None => Err(Box::new(OtherError::new(
                ErrorKind::Other,
                "No Task at Given Index",
            ))),
        }
    }

    fn change_task_priority(
        &self,
        index: usize,
        new_priority: String,
    ) -> Result<TaskList, Box<dyn Error>> {
        match self.tasks.iter().nth(index) {
            Some(task_ref) => Ok(TaskList {
                tasks: self
                    .tasks
                    .update(index, task_ref.clone().change_priority(&new_priority)?),
            }),
            None => Err(Box::new(OtherError::new(
                ErrorKind::Other,
                "No Task at Given Index",
            ))),
        }
    }

    fn select_task_by_id(&self, id: Uuid) -> Result<Task, Box<dyn Error>> {
        let search_task = self.clone().tasks.into_iter().find(|tasks| tasks.id == id);
        match search_task {
            Some(task) => return Ok(task.clone().to_owned()),
            None => {
                return Err(Box::new(OtherError::new(
                    ErrorKind::Other,
                    "No Task with given ID",
                )))
            }
        }
    }
}

pub fn create_task_list() -> TaskList {
    return TaskList { tasks: vector![] };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    mod task_list {
        use super::*;

        #[test]
        fn task_list_creation() {
            let test_task_list = create_task_list();
            assert_eq!(test_task_list, TaskList { tasks: vector![] });
        }

        #[test]
        fn child_task_addition() -> Result<(), Box<dyn Error>> {
            let empty_task_list = create_task_list();
            let single_task_list = &empty_task_list.add_nil_task();
            let test_task = &single_task_list.tasks[0];
            assert!(test_task.name == "Default Task");
            assert!(test_task.completed == false);
            assert!(test_task.priority == PriEnum::Optional);
            assert!(
                test_task.id == Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()
            );
            assert!(&single_task_list.tasks[0] == test_task);
            return Ok(());
        }

        #[test]
        fn task_successful_search_by_id_test() -> Result<(), Box<dyn Error>> {
            let empty_list = create_task_list();
            let single_nil_task_list = &empty_list.add_nil_task();
            let test_search_task = single_nil_task_list.select_task_by_id(Uuid::nil());
            assert!(
                test_search_task.unwrap()
                    == Task {
                        id: Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
                        name: String::from("Default Task"),
                        completed: false,
                        priority: PriEnum::Optional
                    }
            );

            return Ok(());
        }

        #[test]
        fn task_failed_search_by_id_test() -> Result<(), Box<dyn Error>> {
            let test_task_list = create_task_list();
            let test_search_task = test_task_list
                .select_task_by_id(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap());
            assert!(test_search_task.unwrap_err().to_string() == "No Task with given ID");
            return Ok(());
        }

        #[test]
        fn task_print_fail_test() {
            let test_task_list = create_task_list();
            let error = &test_task_list.print_task_list().unwrap_err();
            assert_eq!(error.to_string(), "list is empty");
        }

        #[test]
        fn task_print_successful_test() {
            let empty_task_list = create_task_list();
            let single_task_list = &empty_task_list.add_nil_task();

            let success = &single_task_list.print_task_list().unwrap();

            assert_eq!(
                format!("{}", success.to_string()),
                "name,priority,completed,ID\nDefault Task,Optional,false,00000000-0000-0000-0000-000000000000\n"
            );
        }

        #[test]
        fn failing_task_removal_test() {
            let test_task_list = create_task_list();
            let error = &test_task_list.remove_task(0).unwrap_err();
            assert_eq!(error.to_string(), "No Task at Given Index");
        }

        #[test]
        fn successful_task_removal_test() {
            let empty_task_list = create_task_list();
            let single_task_list = &empty_task_list.create_task();
            let good_result = &single_task_list.remove_task(0).unwrap();
            assert!(good_result.tasks.is_empty());
        }

        #[test]
        fn failing_task_completion_bad_index_test() {
            let test_task_list = create_task_list();
            let error = &test_task_list.toggle_task_completion_status(0).unwrap_err();
            assert_eq!(error.to_string(), "No Task at Given Index");
        }

        #[test]
        fn successful_task_completion_test() {
            let empty_task_list = create_task_list();
            let single_task_list = &empty_task_list.create_task();
            let good_result = &single_task_list.toggle_task_completion_status(0).unwrap();
            assert!(good_result.tasks[0].completed == true);
        }

        #[test]
        fn successful_task_reopen_test() {
            let mut test_task_list = create_task_list();
            test_task_list.tasks.push_front(Task {
                completed: true,
                ..Default::default()
            });
            let updated_task_list = &test_task_list.toggle_task_completion_status(0).unwrap();
            assert_eq!(updated_task_list.tasks[0].completed, false);
        }

        #[test]
        fn failing_task_rename_bad_index_test() {
            let test_task_list = create_task_list();
            let error = &test_task_list
                .rename_task(0, "Change Test".to_string())
                .unwrap_err();
            assert_eq!(error.to_string(), "No Task at Given Index");
        }

        #[test]
        fn successful_task_rename_test() {
            let empty_task_list = create_task_list();
            let single_task_list = &empty_task_list.create_task();
            let good_result = &single_task_list
                .rename_task(0, "Changed Task".to_string())
                .unwrap();
            assert!(good_result.tasks[0].name == "Changed Task".to_string());
        }

        #[test]
        fn failing_task_reprioritize_bad_index_test() {
            let test_task_list = create_task_list();
            let error = &test_task_list
                .change_task_priority(0, "Optional".to_string())
                .unwrap_err();
            assert_eq!(error.to_string(), "No Task at Given Index");
        }

        #[test]
        fn failing_task_reprioritize_bad_priority_test() {
            let empty_task_list = create_task_list();
            let single_task_list = &empty_task_list.create_task();
            let error = &single_task_list
                .change_task_priority(0, "bad priority".to_string())
                .unwrap_err();
            assert_eq!(error.to_string(), "invalid priority".to_string());
        }

        #[test]
        fn successful_task_reprioritize_test() {
            let empty_task_list = create_task_list();
            let single_task_list = &empty_task_list.create_task();
            let changed_task_list = &single_task_list
                .change_task_priority(0, "low".to_string())
                .unwrap();
            assert_eq!(changed_task_list.tasks[0].priority, PriEnum::Low);
        }
    }
}
