use diesel::prelude::*;
use crate::config::Config;
pub mod schema;
pub mod config;

pub struct Store{
    pub conn: PgConnection
}

impl Store {
    pub fn default() -> Result<Self, ConnectionError> {
        let config = config::Config::default();
        let conn = PgConnection::establish(&config.db_url)?;
        Ok(Self { conn })
    }
}

impl Store {
    pub fn create_user(&self){

    }
    pub fn create_website(&self) -> String {
        "123".to_string()
        
    }
}