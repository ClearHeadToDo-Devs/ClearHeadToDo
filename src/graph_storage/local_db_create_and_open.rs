use std::{error::Error, ffi::OsString};

pub trait GraphDatabaseCreation {
    fn create_database(location: Option<OsString>) -> Self;
    fn read_database(location: OsString) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}
