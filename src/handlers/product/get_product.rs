use crate::{models::product::Product, result::MResult};
use axum::{extract::Path, http::StatusCode, Json};

pub async fn get_product(Path(product_id): Path<i32>) -> (StatusCode, Json<MResult<Product>>) {
    let result = Product::get_by_id_from_db(product_id);
    match result {
        Ok(product) => {
            let r = MResult {
                status: StatusCode::OK.as_u16(),
                description: None,
                value: Some(product),
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
    use crate::handlers::product::{add_product::add_product, get_product::get_product};
    use crate::models::product::NewProduct;
    use axum::extract::Path;
    use axum::http::StatusCode;
    use axum::Json;
    use rand::{distributions::Alphanumeric, Rng};

    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_get_product() {
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
        let product_id = add_result.1.value.unwrap();
        let path = Path(product_id as i32);
        let get_result = get_product(path).await;
        assert!(
            StatusCode::OK == get_result.0 && get_result.1.value.is_some(),
            "Не удалось получить продукт"
        );
        assert!(
            get_result.1.value.clone().unwrap().name == product_name,
            "Не удалось получить продукт"
        );
    }
}
