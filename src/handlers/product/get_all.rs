use crate::{models::product::Product, result::MResult};
use axum::{extract::State, http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;

pub async fn get_all(State(pool): State<Pool>) -> (StatusCode, Json<MResult<Vec<Product>>>) {
    match Product::get_all_from_db(pool).await {
        Ok(products) => {
            let r = MResult {
                status: StatusCode::OK.as_u16(),
                description: None,
                value: Some(products),
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

    use crate::handlers::product::get_all::get_all;
    use axum::{extract::State, http::StatusCode};
    use deadpool_diesel::postgres::{Manager, Pool, Runtime};
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_get_all_product() {
        dotenv().ok();
        let manager = Manager::new(
            env::var("DATABASE_URL").expect("PORT must be set"),
            Runtime::Tokio1,
        );
        let pool = Pool::builder(manager).max_size(8).build().unwrap();
        let get_results = get_all(State(pool)).await;
        assert!(
            StatusCode::OK == get_results.0 && get_results.1.value.clone().is_some(),
            "Не удалось получить все продукты"
        );
    }
}
