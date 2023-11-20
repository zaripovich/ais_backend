use crate::schema::*;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
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
    pub async fn add_into_db(
        pool: Pool,
        product: NewProduct,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        use crate::schema::products::dsl::*;

        let connection = pool.get().await?;
        let result = connection
            .interact(move |connection| {
                let query = diesel::insert_into(products).values(product).returning(id);
                query.get_result::<i32>(connection)
            })
            .await??;
        Ok(result)
    }
}

impl Product {
    pub async fn get_by_id_from_db(
        pool: Pool,
        _id: i32,
    ) -> Result<Product, Box<dyn std::error::Error>> {
        use crate::schema::products::dsl::*;
        let connection = pool.get().await?;
        let result = connection
            .interact(move |connection| {
                let query = products.filter(id.eq(_id));
                query.get_result::<Product>(connection)
            })
            .await??;
        Ok(result)
    }

    pub async fn get_all_from_db(pool: Pool) -> Result<Vec<Product>, Box<dyn std::error::Error>> {
        use crate::schema::products::dsl::*;
        let connection = pool.get().await?;
        let result = connection
            .interact(move |connection| {
                let query = products.select(Product::as_select());
                query.get_results::<Product>(connection)
            })
            .await??;
        Ok(result)
    }

    /* pub async fn update_into_db(pool: Pool, value: NewProduct) -> Result<usize, Error> {
        use crate::schema::products::dsl::*;
        let connection = &mut establish_connection();
        let response = products.filter(id.eq(self.id));
        diesel::update(response).set(self).execute(connection)
    } */
}
