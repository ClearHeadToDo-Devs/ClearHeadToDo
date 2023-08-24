use crate::{Action, Priority, RelationshipVariant};
use indradb::{
    BulkInsertItem, Database, Datastore, Edge, EdgeProperties, Identifier, Json, MemoryDatastore,
    NamedProperty, Query, SpecificVertexQuery, Vertex, VertexProperties, VertexProperty,
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

    fn add_action(&self, action: &Action) -> Result<Uuid, Box<dyn Error>> {
        let action_vertex: VertexProperties = action.into();
        let vertex_query: SpecificVertexQuery =
            SpecificVertexQuery::single(action_vertex.vertex.id);

        self.db.create_vertex(&action_vertex.vertex)?;
        self.db.set_properties(
            vertex_query.clone(),
            action_vertex.props[0].name,
            &action_vertex.props[0].value,
        )?;
        self.db.set_properties(
            vertex_query.clone(),
            action_vertex.props[1].name,
            &action_vertex.props[1].value,
        )?;
        self.db.set_properties(
            vertex_query.clone(),
            action_vertex.props[2].name,
            &action_vertex.props[2].value,
        )?;

        Ok(action_vertex.vertex.id)
    }

    fn update_action(
        &self,
        target_field: ActionField,
        action_ref: &Action,
    ) -> Result<(), Box<dyn Error>> {
        let action_query = SpecificVertexQuery::single(action_ref.get_id());
        let updated_outcome = match target_field {
            ActionField::Name => self.db.set_properties(
                action_query,
                Identifier::new("name").unwrap(),
                &Json::new(Value::String(action_ref.get_name())),
            )?,
            ActionField::Priority => self.db.set_properties(
                action_query,
                Identifier::new("priority").unwrap(),
                &Json::new(action_ref.get_priority().into()),
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

    fn create_relationship(
        &self,
        outbound_id: Uuid,
        inbound_id: Uuid,
        rel_var: RelationshipVariant,
    ) -> Result<bool, Box<dyn Error>> {
        let relationship: Edge = Edge::new(
            outbound_id,
            Identifier::new(rel_var.to_string()).unwrap(),
            inbound_id,
        );

        let result = self.db.create_edge(&relationship)?;

        Ok(result)
    }
    fn sync(&self) -> Result<(), Box<dyn Error>> {
        Ok(self.db.sync()?)
    }
}
impl From<&Action> for VertexProperties {
    fn from(value: &Action) -> Self {
        VertexProperties::new(
            Vertex::with_id(value.get_id(), Identifier::new("action").unwrap()),
            vec![
                NamedProperty::new(
                    Identifier::new("name").unwrap(),
                    Json::new(Value::String(value.get_name())),
                ),
                NamedProperty::new(
                    Identifier::new("priority").unwrap(),
                    Json::new(value.get_priority().into()),
                ),
                NamedProperty::new(
                    Identifier::new("completed").unwrap(),
                    Json::new(Value::Bool(value.get_completion_status())),
                ),
            ],
        )
    }
}
impl From<&Action> for Vertex {
    fn from(value: &Action) -> Self {
        let identifier = Identifier::new("action").unwrap();
        Vertex::new(identifier)
    }
}
impl From<Priority> for Value {
    fn from(value: Priority) -> Self {
        match value {
            Priority::Critical => Value::Number(1.into()),
            Priority::High => Value::Number(2.into()),
            Priority::Medium => Value::Number(3.into()),
            Priority::Low => Value::Number(4.into()),
            Priority::Optional => Value::Number(5.into()),
        }
    }
}
enum ActionField {
    Name = 1,
    Priority = 2,
    Completed = 3,
}
#[cfg(test)]
mod tests {
    use crate::RelationshipVariant;

    use super::*;
    use std::fs;

    mod updates {
        use super::*;
        #[test]
        fn add_default_action() {
            let interactor = create_local_interactor(None);
            let action = Action::new("test", None);

            let new_vertex_id = interactor.add_action(&action).unwrap();

            assert!(!new_vertex_id.is_nil())
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
    fn remove_an_action() {
        let interactor = create_local_interactor(None);
        let action = Action::new("test", None);
        interactor.add_action(&action).unwrap();

        let outcome = interactor.delete_action(action.get_id());

        assert!(outcome.is_ok())
    }

    #[test]
    fn create_relationship_between_actions() {
        let interactor = create_local_interactor(None);
        let action_1 = Action::new("test", None);
        let action_2 = Action::new("test_2", None);

        let outbound_id = interactor.add_action(&action_1).unwrap();
        let inbound_id = interactor.add_action(&action_2).unwrap();

        let relationship_created = interactor
            .create_relationship(outbound_id, inbound_id, RelationshipVariant::Related)
            .unwrap();

        assert!(relationship_created)
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
