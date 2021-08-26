use super::TaskList;
use super::task::Task;
use uuid::Uuid;


impl TaskList {
    pub fn add_nil_task(self) -> Self {
        let mut new_list = self.clone();
        let new_task: Task = Task {
            id: Uuid::nil(),
            ..Default::default()
        };
        new_list.tasks.push_back(new_task);
        return new_list;
    }
}
