use crate::db::establish_connection;
use crate::schema::*;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
}
#[derive(Deserialize, Insertable, Queryable, Debug)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub name: String,
}

impl NewRole {
    pub fn add_into_db(&self) -> Result<usize, Error> {
        let connection = &mut establish_connection();
        diesel::insert_into(roles::table)
            .values(self)
            .execute(connection)
    }
}

impl Role {
    pub fn get_from_db(_id: Option<i32>, _name: Option<String>) -> Result<Role, Error> {
        use crate::schema::roles::dsl::*;
        let connection = &mut establish_connection();
        return roles
            .filter(
                id.eq(_id.unwrap_or(-1))
                    .or(name.eq(_name.unwrap_or("".to_string()))),
            )
            .first(connection);
    }

    pub fn update_into_db(&self) -> Result<usize, Error> {
        use crate::schema::roles::dsl::*;
        let connection = &mut establish_connection();
        let response = roles.filter(id.eq(self.id));
        diesel::update(response).set(self).execute(connection)
    }
}
