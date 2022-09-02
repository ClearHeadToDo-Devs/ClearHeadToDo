use im::Vector;
use crate::Action;
use std::fmt::Formatter;
use std::{error::Error, fmt::Display};
use uuid::Uuid;


pub trait ActionListManipulation {
    fn create_new(&self) -> Self;
    fn remove(&self, index: usize) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn rename(&self, index: usize, new_name: String) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn toggle_completion_status(&self, index: usize) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn change_priority(
        &self,
        index: usize,
        new_priority: String,
    ) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn select_by_id(&self, id: Uuid) -> Result<Action, Box<dyn Error>>;
    fn get_id_by_index(&self, index: usize) -> Result<Uuid, Box<dyn Error>>;
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::item::action_manipulation::tests::TestStruct;
    use crate::{ActionListManipulation, ActionManipulation, Action};
    use uuid::Uuid;

    impl ActionListManipulation for Vec<TestStruct> {
        fn create_new(&self) -> Self {
            let mut new_list = self.clone();
            new_list.push(TestStruct::default());

            return new_list;
        }


        fn remove(&self, index: usize) -> Result<Self, Box<dyn Error>> {
            let mut new_list = self.clone();
            match self.iter().nth(index) {
                Some(_test_struct_ref) => {
                    let _new_split = new_list.split_off(0);
                    Ok(new_list)
                }
                None => Err("invalid index".into()),
            }
        }

        fn rename(
            &self,
            index: usize,
            new_name: String,
        ) -> Result<Self, Box<dyn Error>> {
            let mut new_list = self.clone();
            match self.iter().nth(index) {
                Some(_) => {
                    new_list[index].name = new_name;
                    Ok(new_list)
                }
                None => Err("invalid index".into()),
            }
        }

        fn toggle_completion_status(
            &self,
            index: usize,
        ) -> Result<Self, Box<dyn Error>> {
            let mut cloned_list = self.clone();
            match self.iter().nth(index) {
                Some(test_struct_ref) => {
                    cloned_list[index].completed = !test_struct_ref.completed;
                    Ok(cloned_list)
                }
                None => Err("invalid index".into()),
            }
        }

        fn change_priority(
            &self,
            index: usize,
            new_priority: String,
        ) -> Result<Self, Box<dyn Error>> {
            let mut cloned_list = self.clone();
            match self.iter().nth(index) {
                Some(_) => {
                    let updated_action = self[index].change_priority(&new_priority)?;
                    cloned_list[index] = updated_action;
                    return Ok(cloned_list);
                }
                None => Err("invalid index".into()),
            }
        }

        fn get_id_by_index(&self, index: usize) -> Result<Uuid, Box<dyn Error>> {
            match self.iter().nth(index) {
                Some(test_struct_ref) => Ok(test_struct_ref.get_id()),
                None => Err("invalid index".into()),
            }
        }

        fn select_by_id(&self, id: Uuid) -> Result<Action, Box<dyn Error>> {
            match self.iter().find(|test_struct_ref| test_struct_ref.get_id() == id) {
                Some(_test_struct_ref) => Ok(Action::default()),
                None => Err("invalid id".into()),
            }
        }
    }

    #[test]
    fn create_new() {
        let test_struct_list: Vec<TestStruct> = Vec::new();

        let new_test_struct_list = test_struct_list.create_new();

        assert_eq!(new_test_struct_list.len(), 1);
    }

    #[test]
    fn get_list() {
        let test_struct_list: Vec<TestStruct> = Vec::new();

        let new_test_struct_list = test_struct_list.create_new();

        assert!(format!("{:?}", new_test_struct_list).is_empty() == false);
    }

    #[test]
    fn remove() {
        let mut test_struct_list: Vec<TestStruct> = Vec::new();
        test_struct_list.push(TestStruct::default());

        let new_test_struct_list = test_struct_list.remove(0).unwrap();

        assert_eq!(new_test_struct_list.len(), 0);
    }

    #[test]
    fn failed_remove() {
        let test_struct_list: Vec<TestStruct> = Vec::new();

        let new_test_struct_list = test_struct_list.remove(0);

        assert!(new_test_struct_list.is_err());
    }

    #[test]
    fn rename() {
        let test_struct_list: Vec<TestStruct> = Vec::new().create_new();

        let new_test_struct_list = test_struct_list.rename(0, "new name".to_string()).unwrap();

        assert_eq!(new_test_struct_list.get(0).unwrap().name, "new name");
    }

    #[test]
    fn failed_rename() {
        let test_struct_list: Vec<TestStruct> = Vec::new();

        let new_test_struct_list = test_struct_list.rename(0, "new name".to_string());

        assert!(new_test_struct_list.is_err());
    }

    #[test]
    fn toggle_completion_status() {
        let test_struct_list: Vec<TestStruct> = Vec::new().create_new();

        let new_test_struct_list = test_struct_list.toggle_completion_status(0).unwrap();

        assert_eq!(new_test_struct_list.get(0).unwrap().completed, true);
    }

    #[test]
    fn failed_toggle_completion_status() {
        let test_struct_list: Vec<TestStruct> = Vec::new();

        let new_test_struct_list = test_struct_list.toggle_completion_status(0);

        assert!(new_test_struct_list.is_err());
    }

    #[test]
    fn change_priority() {
        let test_struct_list: Vec<TestStruct> = Vec::new().create_new();

        let new_test_struct_list = test_struct_list.change_priority(0, "High".to_string()).unwrap();

        assert_eq!(new_test_struct_list.get(0).unwrap().priority, crate::Priority::High);
    }

    #[test]
    fn failed_change_priority() {
        let test_struct_list: Vec<TestStruct> = Vec::new();

        let new_test_struct_list = test_struct_list.change_priority(0, "High".to_string());

        assert!(new_test_struct_list.is_err());
    }

    #[test]
    fn get_id_by_index() {
        let test_struct_list: Vec<TestStruct> = Vec::new().create_new();

        let id = test_struct_list.get_id_by_index(0).unwrap();

        assert_eq!(id, test_struct_list.get(0).unwrap().get_id());
    }

    #[test]
    fn failed_get_id_by_index() {
        let test_struct_list: Vec<TestStruct> = Vec::new();

        let id = test_struct_list.get_id_by_index(0);

        assert!(id.is_err());
    }

    #[test]
    fn select_by_id() {
        let test_struct_list: Vec<TestStruct> = Vec::new().create_new();

        let id = test_struct_list.get(0).unwrap().get_id();

        let test_struct = test_struct_list.select_by_id(id).unwrap();

        assert_eq!(test_struct.get_name(), "Default Action");
    }

    #[test]
    fn failed_select_by_id() {
        let test_struct_list: Vec<TestStruct> = Vec::new();

        let test_struct = test_struct_list.select_by_id(Uuid::new_v4());

        assert!(test_struct.is_err());
    }
}
