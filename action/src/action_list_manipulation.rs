use crate::action_manipulation::ActionManipulation;
use std::error::Error;
use uuid::Uuid;


pub trait ActionListManipulation {
    type Child: ActionManipulation;
    fn create_new(&self) -> Self;
    fn get_list(&self) -> Result<String, Box<dyn Error>>;
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
    fn select_by_id(&self, id: Uuid) -> Result<Self::Child, Box<dyn Error>>;
    fn get_id_by_index(&self, index: usize) -> Result<Uuid, Box<dyn Error>>;
}

#[cfg(test)]
mod tests {
    use im::Vector;
    use std::error::Error;
    use crate::item::action_manipulation::tests::TestStruct;
    use crate::{ActionListManipulation, ActionManipulation};
    use uuid::Uuid;

    impl ActionListManipulation for Vector<TestStruct> {
        type Child = TestStruct;

        fn create_new(&self) -> Self {
            let mut new_list = self.clone();
            new_list.push_back(TestStruct::default());

            return new_list;
        }

        fn get_list(&self) -> Result<String, Box<dyn Error>> {
            let mut list = String::new();
            let mut index = 0;

            for test_struct in self.iter() {
                list.push_str(&format!("{},{}", index, test_struct.to_string()));
            }

            Ok(list)
        }

        fn remove(&self, index: usize) -> Result<Self, Box<dyn Error>> {
            match self.iter().nth(index) {
                Some(_test_struct_ref) => {
                    let (mut left_side, mut right_side) = self.clone().split_at(index);
                    right_side.pop_front().unwrap();
                    left_side.append(right_side);
                    Ok(left_side)
                }
                None => Err("invalid index".into()),
            }
        }

        fn rename(
            &self,
            index: usize,
            new_name: String,
        ) -> Result<Self, Box<dyn Error>> {
            match self.iter().nth(index) {
                Some(test_struct_ref) => {
                    return Ok(self.update(index, test_struct_ref.rename(&new_name)))
                }
                None => Err("invalid index".into()),
            }
        }

        fn toggle_completion_status(
            &self,
            index: usize,
        ) -> Result<Self, Box<dyn Error>> {
            match self.iter().nth(index) {
                Some(test_struct_ref) => {
                    return Ok(self.update(index, test_struct_ref.toggle_completion_status()))
                }
                None => Err("invalid index".into()),
            }
        }

        fn change_priority(
            &self,
            index: usize,
            new_priority: String,
        ) -> Result<Self, Box<dyn Error>> {
            match self.iter().nth(index) {
                Some(test_struct_ref) => {
                    return Ok(self.update(index, test_struct_ref.change_priority(&new_priority)?))
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

        fn select_by_id(&self, id: Uuid) -> Result<Self::Child, Box<dyn Error>> {
            match self.iter().find(|test_struct_ref| test_struct_ref.get_id() == id) {
                Some(test_struct_ref) => Ok(test_struct_ref.clone()),
                None => Err("invalid id".into()),
            }
        }
}
}
