use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
pub trait DataBase {}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(PartialEq)]
pub struct DBResult {
    pub result: bool,
    pub description: String,
}
