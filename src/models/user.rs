use crate::db::establish_connection;
use crate::result::MResult;
use crate::schema::*;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Selectable, Queryable, AsChangeset, Debug)]
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
    pub fn add_into_db(&self) -> bool {
        let connection = &mut establish_connection();
        let result = diesel::insert_into(users::table)
            .values(self)
            .execute(connection);
        match result {
            Ok(result) => return result > 0,
            Err(_) => return false,
        }
    }
}

impl User {
    pub fn get_from_db(_id: i32) -> Result<User, Error> {
        use crate::schema::users::dsl::*;
        let connection = &mut establish_connection();
        return users.filter(id.eq(_id)).first(connection);
    }

    pub fn update_into_db(&self) -> MResult<String> {
        use crate::schema::users::dsl::*;
        let connection = &mut establish_connection();
        let response = users.filter(id.eq(self.id));
        let result = diesel::update(response).set(self).execute(connection);
        match result {
            Ok(_) => {
                return MResult {
                    status: 200,
                    description: Some("Успешно".to_string()),
                    value: None,
                }
            }
            Err(err) => {
                return MResult {
                    status: 401,
                    description: Some(err.to_string()),
                    value: None,
                }
            }
        }
    }
}
