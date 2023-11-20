use crate::schema::*;
use chrono::naive::NaiveDateTime;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
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
    pub async fn add_into_db(
        pool: Pool,
        order: NewOrder,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        use crate::schema::orders::dsl::*;
        let connection = pool.get().await?;
        let result = connection
            .interact(move |connection| {
                let query = diesel::insert_into(orders).values(order).returning(id);
                query.get_result::<i32>(connection)
            })
            .await??;
        Ok(result)
    }
}

impl Order {
    pub async fn get_by_id_from_db(
        pool: Pool,
        _id: i32,
    ) -> Result<Order, Box<dyn std::error::Error>> {
        use crate::schema::orders::dsl::*;
        let connection = pool.get().await?;
        let result = connection
            .interact(move |connection| {
                let query = orders.filter(id.eq(_id));
                query.get_result::<Order>(connection)
            })
            .await??;
        Ok(result)
    }

    pub async fn delete_from_db(pool: Pool, _id: i32) -> Result<usize, Box<dyn std::error::Error>> {
        use crate::schema::orders::dsl::*;
        let connection = pool.get().await?;
        let result = connection
            .interact(move |connection| {
                let query = diesel::delete(orders.filter(id.eq(_id)));
                query.execute(connection)
            })
            .await??;
        Ok(result)
    }
}
