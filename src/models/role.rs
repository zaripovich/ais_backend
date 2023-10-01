use crate::db::establish_connection;
use crate::result::MResult;
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

impl Clone for Role {
    fn clone(&self) -> Role {
        Role {
            id: self.id,
            name: self.name.to_string(),
        }
    }
}

#[derive(Deserialize, Insertable, Queryable, Debug)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub name: String,
}

impl NewRole {
    pub fn add_into_db(&self) -> bool {
        let connection = &mut establish_connection();
        let result = diesel::insert_into(roles::table)
            .values(self)
            .execute(connection);
        match result {
            Ok(result) => return result > 0,
            Err(_) => return false,
        }
    }
}

impl Role {
    pub fn get_from_db(_id: Option<i32>, _name: Option<String>) -> Result<Role, Error> {
        use crate::schema::roles::dsl::*;
        let connection = &mut establish_connection();
        return roles
            .filter(id.eq(_id.unwrap()).or(name.eq(_name.unwrap())))
            .first(connection);
    }

    pub fn update_into_db(&self) -> MResult<String> {
        use crate::schema::roles::dsl::*;
        let connection = &mut establish_connection();
        let response = roles.filter(id.eq(self.id));
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
