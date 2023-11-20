use crate::models::order::Order;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use deadpool_diesel::postgres::Pool;

pub async fn delete_order(State(pool): State<Pool>, Path(order_id): Path<i32>) -> StatusCode {
    if let Ok(_) = Order::delete_from_db(pool, order_id).await {
        return StatusCode::OK;
    }
    StatusCode::INTERNAL_SERVER_ERROR
}

#[cfg(test)]
mod tests {

    use std::env;

    use crate::handlers::order::{
        add_order::{add_order, AddParametrs},
        delete_order::delete_order,
    };
    use crate::handlers::product::add_product::add_product;
    use crate::models::product::NewProduct;
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::Json;
    use deadpool_diesel::postgres::{Manager, Pool, Runtime};
    use dotenvy::dotenv;
    use rand::{distributions::Alphanumeric, Rng};

    #[tokio::test]
    async fn test_delete_order() {
        dotenv().ok();
        let manager = Manager::new(
            env::var("DATABASE_URL").expect("PORT must be set"),
            Runtime::Tokio1,
        );
        let pool = Pool::builder(manager).max_size(8).build().unwrap();

        let product_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(29)
            .map(char::from)
            .collect();
        let test_product = NewProduct {
            name: product_name.clone(),
            price: 1000,
        };
        let add_product_result = add_product(State(pool.clone()), Json(test_product)).await;

        assert!(
            StatusCode::OK == add_product_result.0,
            "Не удалось добавить продукт"
        );
        let product_id = add_product_result.1.value.unwrap();

        let parametrs = AddParametrs {
            product_id: product_id,
            table_id: 1,
        };
        let add_order_result = add_order(State(pool.clone()), Json(parametrs)).await;
        assert!(
            StatusCode::OK == add_order_result.0 && add_order_result.1.value.is_some(),
            "Не удалось добавить заказ"
        );

        let order_id = add_order_result.1.value.unwrap();
        let delete_result = delete_order(State(pool), Path(order_id)).await;
        assert!(StatusCode::OK == delete_result, "Не удалось удалить заказ");
    }
}
