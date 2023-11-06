use crate::models::order::Order;
use crate::schema::*;
use crate::{db::establish_connection, models::product::Product};
use diesel::prelude::*;
use diesel::result::Error;
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
    pub fn update_active_status(_id: i32, _active: bool) -> Result<usize, Error> {
        use crate::schema::tables::dsl::*;
        let connection = &mut establish_connection();
        diesel::update(tables.filter(id.eq(_id)))
            .set(active.eq(_active))
            .execute(connection)
    }

    pub fn paid(_id: i32) -> Result<usize, Error> {
        let connection = &mut establish_connection();
        {
            use crate::schema::tables::dsl::*;
            if let Err(err) = diesel::update(tables.filter(id.eq(_id)))
                .set(active.eq(false))
                .execute(connection)
            {
                return Err(err);
            }
        }
        use crate::schema::orders::dsl::*;
        diesel::update(orders.filter(table_id.eq(_id).and(active.eq(true))))
            .set(active.eq(false))
            .execute(connection)
    }
}

pub fn get_updates() -> Result<Vec<Table>, Error> {
    let mut result: Vec<Table> = vec![];
    use crate::schema::orders::dsl::*;

    let connection = &mut establish_connection();
    for i in 1..7 {
        let orders_res: Vec<Order> = orders
            .filter(active.eq(true).and(table_id.eq(i)))
            .select(Order::as_select())
            .load(connection)
            .expect("Error loading orders");
        let mut orders_out: Vec<Product> = vec![];
        let active_val: bool = orders_res.len() > 0;
        for order in orders_res {
            use crate::schema::products::dsl::*;
            let mut product_res: Product = products
                .filter(id.eq(order.product_id))
                .select(Product::as_select())
                .first(connection)
                .expect("Error loading products");
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
