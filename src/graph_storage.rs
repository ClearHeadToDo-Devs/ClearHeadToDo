use indradb::{Database, MemoryDatastore};
use rmp_serde::decode::Error as RmpDecodeError;
use std::error::Error;
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

    fn create_in_memory_interactor() -> LocalIndraInteractor {
        LocalIndraInteractor {
            db: MemoryDatastore::new_db(),
        }
    }

    fn sync(&self) -> Result<(), Box<dyn Error>> {
        Ok(self.db.sync()?)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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

    fn create_local_interactor(interactor_path: Option<&str>) -> LocalIndraInteractor {
        LocalIndraInteractor::new(interactor_path)
    }

    fn can_sync_db(interactor: LocalIndraInteractor) -> bool {
        interactor.sync().is_ok()
    }
}
