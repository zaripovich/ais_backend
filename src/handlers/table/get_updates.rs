use crate::models::table::{self, Table};
use crate::result::MResult;
use axum::extract::State;
use axum::{http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;

pub async fn get_updates(State(pool): State<Pool>) -> (StatusCode, Json<MResult<Vec<Table>>>) {
    match table::get_updates(pool).await {
        Ok(table) => {
            let r = MResult {
                status: StatusCode::OK.as_u16(),
                description: None,
                value: Some(table),
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

    use crate::handlers::table::get_updates::get_updates;
    use axum::{extract::State, http::StatusCode};
    use deadpool_diesel::postgres::{Manager, Pool, Runtime};
    use dotenvy::dotenv;
    #[tokio::test]
    async fn test_get_updates() {
        dotenv().ok();
        let manager = Manager::new(
            env::var("DATABASE_URL").expect("PORT must be set"),
            Runtime::Tokio1,
        );
        let pool = Pool::builder(manager).max_size(8).build().unwrap();
        let get_updates_result = get_updates(State(pool)).await;
        assert!(
            StatusCode::OK == get_updates_result.0 && get_updates_result.1.value.clone().is_some(),
            "Не удалось получить обновления"
        );
    }
}
