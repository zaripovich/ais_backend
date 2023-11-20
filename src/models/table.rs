use crate::models::order::Order;
use crate::models::product::Product;
use crate::schema::*;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Selectable, Queryable, AsChangeset, Debug)]
#[diesel(table_name = tables)]
pub struct MyTable {
    pub id: i32,
    pub active: bool,
}

#[derive(Serialize, Clone)]
pub struct Table {
    pub id: i32,
    pub active: bool,
    pub orders: Vec<Product>,
}

impl MyTable {
    pub async fn update_active_status(
        pool: Pool,
        _id: i32,
        _active: bool,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        use crate::schema::tables::dsl::*;
        let connection = pool.get().await?;
        let result = connection
            .interact(move |connection| {
                let query = diesel::update(tables.filter(id.eq(_id))).set(active.eq(_active));
                query.execute(connection)
            })
            .await??;
        Ok(result)
    }

    pub async fn paid(pool: Pool, _id: i32) -> Result<usize, Box<dyn std::error::Error>> {
        let connection = pool.get().await?;
        let result_1 = connection
            .interact(move |connection| {
                use crate::schema::tables::dsl::*;
                let query = diesel::update(tables.filter(id.eq(_id))).set(active.eq(false));
                query.execute(connection)
            })
            .await?;
        if let Err(err) = result_1 {
            return Err(Box::new(err));
        };
        let result = connection
            .interact(move |connection| {
                use crate::schema::orders::dsl::*;
                let query = diesel::update(orders.filter(table_id.eq(_id).and(active.eq(true))))
                    .set(active.eq(false));
                query.execute(connection)
            })
            .await??;
        Ok(result)
    }
}

pub async fn get_updates(pool: Pool) -> Result<Vec<Table>, Box<dyn std::error::Error>> {
    let mut result: Vec<Table> = vec![];
    let connection = pool.get().await?;
    for i in 1..7 {
        let orders_res = connection
            .interact(move |connection| {
                use crate::schema::orders::dsl::*;
                let query = orders
                    .filter(active.eq(true).and(table_id.eq(i)))
                    .select(Order::as_select());
                query.get_results::<Order>(connection)
            })
            .await??;
        let mut orders_out: Vec<Product> = vec![];
        let active_val: bool = orders_res.len() > 0;
        for order in orders_res {
            use crate::schema::products::dsl::*;
            let mut product_res: Product = connection
                .interact(move |connection| {
                    let query = products
                        .filter(id.eq(order.product_id))
                        .select(Product::as_select());
                    query.get_result::<Product>(connection)
                })
                .await??;
            product_res.id = order.id;
            orders_out.push(product_res);
        }
        result.push(Table {
            id: i,
            active: active_val,
            orders: orders_out,
        })
    }
    Ok(result)
}
