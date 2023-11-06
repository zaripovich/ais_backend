use crate::db::establish_connection;
use crate::schema::*;
use chrono::naive::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Selectable, Queryable, Debug, Clone)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i32,
    pub active: bool,
    pub product_id: i32,
    pub table_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable, Queryable, Debug)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub active: bool,
    pub product_id: i32,
    pub table_id: i32,
    pub created_at: NaiveDateTime,
}

impl NewOrder {
    pub fn add_into_db(&self) -> Result<i32, Error> {
        use crate::schema::orders::dsl::*;
        let connection = &mut establish_connection();
        diesel::insert_into(orders)
            .values(self)
            .returning(id)
            .get_result(connection)
    }
}

impl Order {
    pub fn get_by_id_from_db(_id: i32) -> Result<Order, Error> {
        use crate::schema::orders::dsl::*;
        let connection = &mut establish_connection();
        orders.filter(id.eq(_id)).first(connection)
    }

    pub fn update_into_db(_id: i32, _active: bool) -> Result<usize, Error> {
        use crate::schema::orders::dsl::*;
        let connection = &mut establish_connection();
        let response = orders.filter(id.eq(_id));
        diesel::update(response)
            .set(active.eq(_active))
            .execute(connection)
    }

    pub fn delete_from_db(_id: i32) -> Result<usize, Error> {
        use crate::schema::orders::dsl::*;
        let connection = &mut establish_connection();
        let response = orders.filter(id.eq(_id));
        diesel::delete(response).execute(connection)
    }
}
