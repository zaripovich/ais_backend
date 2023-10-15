use crate::db::establish_connection;
use crate::schema::*;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use sqlx;

#[derive(Serialize, Selectable, Queryable, AsChangeset, Debug, Clone, sqlx::FromRow)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: i32,
}

#[derive(Deserialize, Insertable, Queryable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub role: i32,
}

impl NewUser {
    pub fn add_into_db(&self) -> Result<usize, Error> {
        let connection = &mut establish_connection();
        diesel::insert_into(users::table)
            .values(self)
            .execute(connection)
    }
}

impl User {
    pub fn get_by_id_from_db(_id: i32) -> Result<User, Error> {
        use crate::schema::users::dsl::*;
        let connection = &mut establish_connection();
        users.filter(id.eq(_id)).first(connection)
    }

    pub fn get_by_username_from_db(_username: String) -> Result<User, Error> {
        use crate::schema::users::dsl::*;
        let connection = &mut establish_connection();
        users.filter(username.eq(_username)).first(connection)
    }

    pub fn update_into_db(&self) -> Result<usize, Error> {
        use crate::schema::users::dsl::*;
        let connection = &mut establish_connection();
        let response = users.filter(id.eq(self.id));
        diesel::update(response).set(self).execute(connection)
    }
}
