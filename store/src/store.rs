use crate::config::Config;
use diesel::prelude::*;
use crate::schema;      

pub struct Store{
    pub conn: PgConnection
}

impl Store {
    pub fn default() -> Result<Self, ConnectionError> {
        let config =Config::default();
        let conn = PgConnection::establish(&config.db_url)?;
        Ok(Self { conn })
    }
}