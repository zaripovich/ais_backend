use crate::{models::order::NewOrder, result::MResult};
use axum::extract::State;
use axum::{http::StatusCode, Json};
use chrono::Local;
use deadpool_diesel::postgres::Pool;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct AddParametrs {
    pub product_id: i32,
    pub table_id: i32,
}

pub async fn add_order(
    State(pool): State<Pool>,
    Json(payload): Json<AddParametrs>,
) -> (StatusCode, Json<MResult<i32>>) {
    let order = NewOrder {
        active: true,
        product_id: payload.product_id,
        table_id: payload.table_id,
        created_at: Local::now().naive_local(),
    };
    match NewOrder::add_into_db(pool, order).await {
        Ok(val) => {
            let r = MResult {
                status: StatusCode::OK.as_u16(),
                description: None,
                value: Some(val),
            };
            (StatusCode::OK, Json(r))
        }
        Err(err) => {
            let r = MResult {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                description: Some(err.to_string()),
                value: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(r))
        }
    }
}

#[cfg(test)]
mod tests {

    use std::env;

    use crate::handlers::order::add_order::{add_order, AddParametrs};
    use crate::handlers::product::add_product::add_product;
    use crate::models::product::NewProduct;
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::Json;
    use deadpool_diesel::postgres::{Manager, Pool, Runtime};
    use rand::{distributions::Alphanumeric, Rng};

    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_add_order() {
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
    }
}
