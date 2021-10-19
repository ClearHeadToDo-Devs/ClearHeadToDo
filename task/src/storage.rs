use crate::Task;

use serde::ser::{Serialize, SerializeStruct, Serializer};

impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Task", 4)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("priority", &self.priority)?;
        s.serialize_field("completed", &self.completed)?;
        s.serialize_field("id", &self.id)?;
        s.end()
    }
}
