pub mod task;
pub use task::*;

mod helper;
pub use helper::*;

use std::error::Error;
use std::io::{Error as OtherError, ErrorKind};
use im::vector;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct TaskList {
    pub tasks: im::Vector<Task>,
}

pub fn create_task_list() -> TaskList {
    return TaskList { tasks: vector![] };
}

impl TaskList {
    pub fn create_task(self) -> Self {
        let new_task = create_default_task();
        let mut new_list = self.clone();
        new_list.tasks.push_back(new_task);
        return new_list;
    }

    pub fn print_task_list(
        &self,
        mut writer: impl std::io::Write,
    ) -> Result<String, Box<dyn Error>> {
        if self.tasks.is_empty() == true {
            return Err(Box::new(OtherError::new(ErrorKind::Other, "list is empty")));
        } else {
            writeln!(writer, "name,priority,completed,ID")?;
            for task in &self.tasks {
                writeln!(
                    writer,
                    "{name},{priority},{completed},{id}",
                    name = task.name,
                    priority = task.priority,
                    completed = task.completed,
                    id = task.id
                )?;
            }
        }
        Ok("End of List".to_string())
    }

    pub fn remove_task(self, index: usize) -> Result<TaskList, Box<dyn Error>> {
        let index_check_result = self.check_index_bounds(index);
        match index_check_result {
            Ok(checked_index) => {
                let (mut left_side, mut right_side) = self.tasks.split_at(checked_index);
                right_side.pop_front().unwrap();
                left_side.append(right_side);
                Ok(TaskList { tasks: left_side })
            }
            Err(error) => Err(error),
        }
    }

    pub fn rename_task(self, index: usize, new_name: String) -> Result<TaskList, Box<dyn Error>> {
        let index_bounds_result = self.check_index_bounds(index);
        match index_bounds_result {
            Ok(checked_index) => Ok(TaskList {
                tasks: self.tasks.update(
                    checked_index,
                    self.tasks[checked_index].clone().rename(&new_name),
                ),
            }),
            Err(error) => Err(error),
        }
    }

    pub fn complete_task(self, index: usize) -> Result<TaskList, Box<dyn Error>> {
        let index_bounds_result = self.check_index_bounds(index);
        match index_bounds_result {
            Ok(checked_index) => Ok(TaskList {
                tasks: self.tasks.update(
                    checked_index,
                    self.tasks[checked_index].clone().mark_complete()?,
                ),
            }),
            Err(error) => Err(error),
        }
    }

    pub fn change_task_priority(
        self,
        index: usize,
        new_priority: String,
    ) -> Result<TaskList, Box<dyn Error>> {
        let index_bounds_result = self.check_index_bounds(index);
        match index_bounds_result {
            Ok(checked_index) => Ok(TaskList {
                tasks: self.tasks.update(
                    checked_index,
                    self.tasks[checked_index]
                        .clone()
                        .change_priority(&new_priority)?,
                ),
            }),
            Err(error) => Err(error),
        }
    }

    pub fn select_task_by_id(self, id: Uuid) -> Result<Task, Box<dyn Error>> {
        let search_task = self.tasks.into_iter().find(|tasks| tasks.id == id);
        match search_task {
            Some(task) => return Ok(task),
            None => {
                return Err(Box::new(OtherError::new(
                    ErrorKind::Other,
                    "No Task with given ID",
                )))
            }
        }
    }

    pub fn check_index_bounds(&self, index: usize) -> Result<usize, Box<dyn Error>> {
        if index < self.tasks.len() {
            Ok(index)
        } else {
            Err(Box::new(OtherError::new(
                ErrorKind::Other,
                "No Task in that position",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;


    mod task_list {
        use super::*;

        #[test]
        fn task_list_creation_test() {
            let test_task_list = create_task_list();
            assert_eq!(test_task_list, TaskList { tasks: vector![] });
        }

        #[test]
        fn child_task_creation_test() -> Result<(), Box<dyn Error>> {
            let empty_task_list = create_task_list();
            let single_task_list = empty_task_list.add_nil_task();
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
        fn successful_search_tasks_by_index_test() -> Result<(), Box<dyn Error>> {
            let empty_list = create_task_list();
            let single_nil_task_list = empty_list.create_task();
            let successful_bounds_check = single_nil_task_list.check_index_bounds(0).unwrap();
            assert!(successful_bounds_check == 0);
            return Ok(());
        }

        #[test]
        fn task_failed_search_by_index_test() -> Result<(), Box<dyn Error>> {
            let test_task_list = create_task_list();
            let failed_bounds_check = test_task_list
                .check_index_bounds(0)
                .unwrap_err()
                .to_string();
            assert_eq!(failed_bounds_check, "No Task in that position".to_string());
            return Ok(());
        }

        #[test]
        fn task_successful_search_by_id_test() -> Result<(), Box<dyn Error>> {
            let empty_list = create_task_list();
            let single_nil_task_list = empty_list.add_nil_task();
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
            let mut bad_result = Vec::new();
            let error = test_task_list.print_task_list(&mut bad_result).unwrap_err();
            assert_eq!(error.to_string(), "list is empty");
        }

        #[test]
        fn task_print_successful_test() -> Result<(), Box<dyn Error>> {
            let empty_task_list = create_task_list();
            let single_task_list = empty_task_list.add_nil_task();

            let mut good_result = Vec::new();
            let success = single_task_list.print_task_list(&mut good_result).unwrap();

            assert_eq!(
                format!("{}", std::str::from_utf8(&good_result).unwrap().to_string()),
                "name,priority,completed,ID\nDefault Task,Optional,false,00000000-0000-0000-0000-000000000000\n"
            );
            assert_eq!(success, "End of List");
            return Ok(());
        }

        #[test]
        fn failing_task_removal_test() {
            let test_task_list = create_task_list();
            let error = test_task_list.remove_task(0).unwrap_err();
            assert_eq!(error.to_string(), "No Task in that position");
        }

        #[test]
        fn successful_task_removal_test() {
            let empty_task_list = create_task_list();
            let single_task_list = empty_task_list.create_task();
            let good_result = single_task_list.remove_task(0).unwrap();
            assert!(good_result.tasks.is_empty());
        }

        #[test]
        fn failing_task_completion_bad_index_test() {
            let test_task_list = create_task_list();
            let error = test_task_list.complete_task(0).unwrap_err();
            assert_eq!(error.to_string(), "No Task in that position");
        }

        #[test]
        fn failing_task_completion_already_complete_test() {
            let mut test_task_list = create_task_list();
            test_task_list.tasks.push_front(Task {
                completed: true,
                ..Default::default()
            });
            let error = test_task_list.complete_task(0).unwrap_err();
            assert_eq!(error.to_string(), "Task is already completed");
        }

        #[test]
        fn successful_task_completion_test() {
            let empty_task_list = create_task_list();
            let single_task_list = empty_task_list.create_task();
            let good_result = single_task_list.complete_task(0).unwrap();
            assert!(good_result.tasks[0].completed == true);
        }

        #[test]
        fn failing_task_rename_bad_index_test() {
            let test_task_list = create_task_list();
            let error = test_task_list
                .rename_task(0, "Change Test".to_string())
                .unwrap_err();
            assert_eq!(error.to_string(), "No Task in that position");
        }

        #[test]
        fn successful_task_rename_test() {
            let empty_task_list = create_task_list();
            let single_task_list = empty_task_list.create_task();
            let good_result = single_task_list
                .rename_task(0, "Changed Task".to_string())
                .unwrap();
            assert!(good_result.tasks[0].name == "Changed Task".to_string());
        }

        #[test]
        fn failing_task_reprioritize_bad_index_test() {
            let test_task_list = create_task_list();
            let error = test_task_list
                .change_task_priority(0, "Optional".to_string())
                .unwrap_err();
            assert_eq!(error.to_string(), "No Task in that position");
        }

        #[test]
        fn failing_task_reprioritize_bad_priority_test() {
            let empty_task_list = create_task_list();
            let single_task_list = empty_task_list.create_task();
            let error = single_task_list
                .change_task_priority(0, "bad priority".to_string())
                .unwrap_err();
            assert_eq!(error.to_string(), "invalid priority".to_string());
        }

        #[test]
        fn successful_task_reprioritize_test() {
            let empty_task_list = create_task_list();
            let single_task_list = empty_task_list.create_task();
            let changed_task_list = single_task_list
                .change_task_priority(0, "low".to_string())
                .unwrap();
            assert_eq!(changed_task_list.tasks[0].priority, PriEnum::Low);
        }
    }
}
