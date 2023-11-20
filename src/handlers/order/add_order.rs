use crate::{models::order::NewOrder, result::MResult};
use axum::{http::StatusCode, Json};
use chrono::Local;

use serde::Deserialize;
#[derive(Deserialize)]
pub struct AddParametrs {
    pub product_id: i32,
    pub table_id: i32,
}

pub async fn add_order(Json(payload): Json<AddParametrs>) -> (StatusCode, Json<MResult<i32>>) {
    let order = NewOrder {
        active: true,
        product_id: payload.product_id,
        table_id: payload.table_id,
        created_at: Local::now().naive_local(),
    };
    match order.add_into_db() {
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

    use crate::handlers::order::add_order::{add_order, AddParametrs};
    use crate::handlers::product::add_product::add_product;
    use crate::models::product::NewProduct;
    use axum::http::StatusCode;
    use axum::Json;
    use rand::{distributions::Alphanumeric, Rng};

    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_add_order() {
        dotenv().ok();
        let product_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(29)
            .map(char::from)
            .collect();
        let test_product = NewProduct {
            name: product_name.clone(),
            price: 1000,
        };
        let add_product_result = add_product(Json(test_product)).await;

        assert!(
            StatusCode::OK == add_product_result.0,
            "Не удалось добавить продукт"
        );
        let product_id = add_product_result.1.value.unwrap();

        let parametrs = AddParametrs {
            product_id: product_id,
            table_id: 1,
        };
        let add_order_result = add_order(Json(parametrs)).await;

        assert!(
            StatusCode::OK == add_order_result.0 && add_order_result.1.value.is_some(),
            "Не удалось добавить заказ"
        );
    }
}
