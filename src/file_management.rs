use crate::Action;

use serde::{Deserialize, Serialize};
use serde_json::*;
use xdg::*;

#[cfg(test)]
mod test {
    use crate::action_builder::ActionBuilder;

    use super::*;
    use crate::priority::Priority;
    use serde_test::*;
    use uuid::Uuid;

    #[test]
    fn action_list_serialize() {
        let action_list = vec![Action {
            name: "Default Action".to_string(),
            completed: false,
            priority: Priority::Optional,
            id: Uuid::nil(),
        }];

        assert_tokens(
            &action_list.readable(),
            &[
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Action",
                    len: 4,
                },
                Token::Str("name"),
                Token::Str("Default Action"),
                Token::Str("completed"),
                Token::Bool(false),
                Token::Str("priority"),
                Token::UnitVariant {
                    name: "Priority",
                    variant: "Optional",
                },
                Token::Str("id"),
                Token::Str("00000000-0000-0000-0000-000000000000"),
                Token::StructEnd,
                Token::SeqEnd,
            ],
        )
    }
}
