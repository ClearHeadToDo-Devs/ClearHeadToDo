extern crate task;
pub use task::*;

pub mod helper;
pub use helper::*;

pub mod storage;
pub use storage::*;

pub mod task_list_manipulation;
pub use task_list_manipulation::TaskListManipulation;

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

impl TaskListManipulation for TaskList {
    type Child = Task;

    fn create_task_list() -> TaskList {
        return TaskList { tasks: vector![] };
    }
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
