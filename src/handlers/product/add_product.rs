use axum::{http::StatusCode, Json};

use crate::{models::product::NewProduct, result::MResult};
pub async fn add_product(Json(payload): Json<NewProduct>) -> (StatusCode, Json<MResult<i32>>) {
    let product = NewProduct {
        name: payload.name,
        price: payload.price,
    };
    match product.add_into_db() {
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
    use crate::handlers::product::add_product::add_product;
    use crate::models::product::NewProduct;
    use axum::http::StatusCode;
    use axum::Json;
    use dotenvy::dotenv;
    use rand::{distributions::Alphanumeric, Rng};

    #[tokio::test]
    async fn test_add_product() {
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
        let add_result = add_product(Json(test_product)).await;
        assert!(
            StatusCode::OK == add_result.0,
            "Не удалось добавить продукт"
        );
    }
}
