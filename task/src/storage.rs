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

#[cfg(test)]
mod tests {

    use super::*;
    use serde_test::{assert_ser_tokens, Configure, Token};
    use uuid::Uuid;

    #[test]
    fn successfully_serialize_task() {
        let test_task = Task {
            id: Uuid::nil(),
            ..Default::default()
        };

        assert_ser_tokens(
            &test_task.readable(),
            &[
                Token::Struct {
                    name: "Task",
                    len: 4,
                },
                Token::Str("name"),
                Token::Str("Default Task"),
                Token::Str("priority"),
                Token::UnitVariant {
                    name: "PriEnum",
                    variant: "Optional",
                },
                Token::Str("completed"),
                Token::Bool(false),
                Token::Str("id"),
                Token::Str("00000000-0000-0000-0000-000000000000"),
                Token::StructEnd,
            ],
        );
    }
}
