#[cfg(test)]
mod tests {

    use crate::Action;
    use serde_test::{assert_de_tokens, assert_ser_tokens, Configure, Token};
    use uuid::Uuid;

    #[test]
    fn successfully_serialize_task() {
        let test_task = Action {
            id: Uuid::nil(),
            ..Default::default()
        };

        assert_ser_tokens(
            &test_task.readable(),
            &[
                Token::Struct {
                    name: "Action",
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

    #[test]
    fn successfully_deserializing_task() {
        let test_task =  Action {
            id: Uuid::nil(),
            ..Default::default()
        };
    assert_de_tokens(&test_task.readable(), &[
        Token::Struct {name: "Action", len:5},
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
    ])
    }
}
