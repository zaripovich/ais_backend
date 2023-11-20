use crate::{models::product::Product, result::MResult};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use deadpool_diesel::postgres::Pool;

pub async fn get_product(
    State(pool): State<Pool>,
    Path(product_id): Path<i32>,
) -> (StatusCode, Json<MResult<Product>>) {
    match Product::get_by_id_from_db(pool, product_id).await {
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
    use std::env;

    use crate::handlers::product::{add_product::add_product, get_product::get_product};
    use crate::models::product::NewProduct;
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::Json;
    use deadpool_diesel::postgres::{Manager, Pool, Runtime};
    use dotenvy::dotenv;
    use rand::{distributions::Alphanumeric, Rng};

    #[tokio::test]
    async fn test_get_product() {
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
        let add_result = add_product(State(pool.clone()), Json(test_product)).await;

        assert!(
            StatusCode::OK == add_result.0,
            "Не удалось добавить продукт"
        );
        let product_id = add_result.1.value.unwrap();
        let path = Path(product_id as i32);
        let get_result = get_product(State(pool.clone()), path).await;
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
