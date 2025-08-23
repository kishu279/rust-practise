use crate::config::Config;
use diesel::prelude::*;
use std::env::{self, VarError};
pub struct Store {
    pub conn: PgConnection,
}

impl Store {
    pub fn default() -> Result<Self, ConnectionError> {
        let db_url = Config::default().db_url; 
        let conn: PgConnection = PgConnection::establish(&db_url)?;

        Ok(Self {conn})
    }
}