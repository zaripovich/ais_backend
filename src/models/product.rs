use crate::db::establish_connection;
use crate::schema::*;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Selectable, Queryable, AsChangeset, Debug, Clone)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32,
}

#[derive(Deserialize, Insertable, Queryable, Debug)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub price: i32,
}

impl NewProduct {
    pub fn add_into_db(&self) -> Result<usize, Error> {
        let connection = &mut establish_connection();
        diesel::insert_into(products::table)
            .values(self)
            .execute(connection)
    }
}

impl Product {
    pub fn get_by_id_from_db(_id: i32) -> Result<Product, Error> {
        use crate::schema::products::dsl::*;
        let connection = &mut establish_connection();
        products.filter(id.eq(_id)).first(connection)
    }

    pub fn get_all_from_db() -> Result<Vec<Product>, Error> {
        use crate::schema::products::dsl::*;
        let connection = &mut establish_connection();
        products.select(Product::as_select()).load(connection)
    }

    pub fn update_into_db(&self) -> Result<usize, Error> {
        use crate::schema::products::dsl::*;
        let connection = &mut establish_connection();
        let response = products.filter(id.eq(self.id));
        diesel::update(response).set(self).execute(connection)
    }
}
