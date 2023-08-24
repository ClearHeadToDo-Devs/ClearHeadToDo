use crate::Action;
use indradb::{
    Database, Datastore, Identifier, Json, MemoryDatastore, NamedProperty, Query,
    SpecificVertexQuery, Vertex, VertexProperties, VertexProperty,
};
use rmp_serde::decode::Error as RmpDecodeError;
use serde_json::{json, Value};
use std::error::Error;
use uuid::Uuid;
struct LocalIndraInteractor {
    db: Database<MemoryDatastore>,
}

impl LocalIndraInteractor {
    fn new(db_path: Option<&str>) -> Self {
        match db_path {
            None => LocalIndraInteractor {
                db: MemoryDatastore::new_db(),
            },
            Some(db_path) => match MemoryDatastore::read_msgpack_db(db_path) {
                Ok(db) => LocalIndraInteractor { db },
                Err(e) => LocalIndraInteractor {
                    db: MemoryDatastore::create_msgpack_db(db_path),
                },
            },
        }
    }

    fn add_action(&self, action: &Action) -> Result<(), Box<dyn Error>> {
        let action_vertex: Vertex = action.into();
        let action_string: String = action.get_name();
        let priority_int: usize = action.get_priority().into();
        let completed: bool = action.get_completion_status();

        let string_property = NamedProperty::new(
            Identifier::new("name").unwrap(),
            Json::new(Value::String(action_string).into()),
        );
        let priority_property = NamedProperty::new(
            Identifier::new("priority").unwrap(),
            Json::new(Value::Number(priority_int.into())),
        );
        let completion_status_property = NamedProperty::new(
            Identifier::new("completed").unwrap(),
            Json::new(Value::Bool(completed)),
        );

        self.db.create_vertex(&action_vertex)?;
        self.db.set_properties(
            SpecificVertexQuery::single(action_vertex.id),
            string_property.name,
            &string_property.value,
        );
        self.db.set_properties(
            SpecificVertexQuery::single(action_vertex.id),
            priority_property.name,
            &priority_property.value,
        );
        self.db.set_properties(
            SpecificVertexQuery::single(action_vertex.id),
            completion_status_property.name,
            &completion_status_property.value,
        );

        Ok(())
    }

    fn update_action(
        &self,
        target_field: ActionField,
        action_ref: &Action,
    ) -> Result<(), Box<dyn Error>> {
        let action_query = SpecificVertexQuery::single(action_ref.get_id());
        let priority_int: usize = action_ref.get_priority().into();
        let updated_outcome = match target_field {
            ActionField::Name => self.db.set_properties(
                action_query,
                Identifier::new("name").unwrap(),
                &Json::new(Value::String(action_ref.get_name())),
            )?,
            ActionField::Priority => self.db.set_properties(
                action_query,
                Identifier::new("priority").unwrap(),
                &Json::new(Value::Number(priority_int.into())),
            )?,
            ActionField::Completed => self.db.set_properties(
                action_query,
                Identifier::new("completed").unwrap(),
                &Json::new(Value::Bool(action_ref.get_completion_status())),
            )?,
        };

        Ok(updated_outcome)
    }

    fn delete_action(&self, action_id: Uuid) -> Result<(), Box<dyn Error>> {
        let action_query = SpecificVertexQuery::single(action_id);

        self.db.delete(action_query)?;
        Ok(())
    }
    fn sync(&self) -> Result<(), Box<dyn Error>> {
        Ok(self.db.sync()?)
    }
}

impl From<&Action> for Vertex {
    fn from(value: &Action) -> Self {
        let identifier = Identifier::new("action").unwrap();
        Vertex::new(identifier)
    }
}

enum ActionField {
    Name = 1,
    Priority = 2,
    Completed = 3,
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    mod updates {
        use super::*;
        #[test]
        fn add_default_action() {
            let interactor = create_local_interactor(None);
            let action = Action::new("test", None);

            let updated_interactor = interactor.add_action(&action);

            assert!(updated_interactor.is_ok())
        }

        #[test]
        fn update_existing_action() {
            let interactor = create_local_interactor(None);
            let action = Action::new("test", None);
            interactor.add_action(&action).unwrap();

            let outcome = interactor.update_action(ActionField::Name, &action);

            assert!(outcome.is_ok())
        }
    }

    #[test]
    fn remove_action() {
        let interactor = create_local_interactor(None);
        let action = Action::new("test", None);
        interactor.add_action(&action).unwrap();

        let outcome = interactor.delete_action(action.get_id());

        assert!(outcome.is_ok())
    }
    mod creation {
        use super::*;

        #[test]
        fn create_interactor() {
            let ephemeral_interactor = create_local_interactor(None);

            assert!(can_sync_db(ephemeral_interactor))
        }

        #[test]
        fn read_local_db() {
            let local_interactor = create_local_interactor(Some("data/test.db"));

            assert!(can_sync_db(local_interactor))
        }

        #[test]
        fn create_new_db() -> Result<(), Box<dyn Error>> {
            let new_interactor = LocalIndraInteractor::new(Some("data/new.db"));

            assert!(can_sync_db(new_interactor));

            Ok(fs::remove_file("data/new.db")?)
        }

        fn can_sync_db(interactor: LocalIndraInteractor) -> bool {
            interactor.sync().is_ok()
        }
    }

    fn create_local_interactor(interactor_path: Option<&str>) -> LocalIndraInteractor {
        LocalIndraInteractor::new(interactor_path)
    }
}
