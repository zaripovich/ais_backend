use crate::db::establish_connection;
use crate::result::MResult;
use crate::schema::*;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = reports)]
pub struct Report {
    pub id: i32,
    pub description: String,
    pub role: i32,
}

#[derive(Deserialize, Insertable, Queryable, Debug)]
#[diesel(table_name = reports)]
pub struct NewReport {
    pub description: String,
    pub role: i32,
}

impl NewReport {
    pub fn add_into_db(&self) -> bool {
        let connection = &mut establish_connection();
        let result = diesel::insert_into(reports::table)
            .values(self)
            .execute(connection);
        match result {
            Ok(result) => return result > 0,
            Err(_) => return false,
        }
    }
}

impl Report {
    pub fn get_by_id_from_db(_id: i32) -> Result<Report, Error> {
        use crate::schema::reports::dsl::*;
        let connection = &mut establish_connection();
        return reports.filter(id.eq(_id)).first(connection);
    }

    pub fn get_by_role_from_db(_role: i32) -> Result<Vec<Report>, Error> {
        use crate::schema::reports::dsl::*;
        let connection = &mut establish_connection();
        return reports.filter(role.eq(_role)).load::<Report>(connection);
    }

    pub fn update_into_db(&self) -> MResult<String> {
        use crate::schema::reports::dsl::*;
        let connection = &mut establish_connection();
        let response = reports.filter(id.eq(self.id));
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
