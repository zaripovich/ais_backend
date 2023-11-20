use crate::models::order::Order;
use axum::{extract::Path, http::StatusCode};

pub async fn delete_order(Path(order_id): Path<i32>) -> StatusCode {
    if let Ok(_) = Order::delete_from_db(order_id) {
        return StatusCode::OK;
    }
    StatusCode::INTERNAL_SERVER_ERROR
}

#[cfg(test)]
mod tests {

    use crate::handlers::order::{
        add_order::{add_order, AddParametrs},
        delete_order::delete_order,
    };
    use crate::handlers::product::add_product::add_product;
    use crate::models::product::NewProduct;
    use axum::extract::Path;
    use axum::http::StatusCode;
    use axum::Json;
    use rand::{distributions::Alphanumeric, Rng};

    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_delete_order() {
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

        let order_id = add_order_result.1.value.unwrap();
        let delete_result = delete_order(Path(order_id)).await;
        assert!(StatusCode::OK == delete_result, "Не удалось удалить заказ");
    }
}
