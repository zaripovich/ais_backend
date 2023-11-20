use crate::{models::table::MyTable, result::MResult};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use deadpool_diesel::postgres::Pool;

pub async fn paid(
    State(pool): State<Pool>,
    Path(table_id): Path<i32>,
) -> (StatusCode, Json<MResult<String>>) {
    match MyTable::paid(pool, table_id).await {
        Ok(_) => {
            let r = MResult {
                status: StatusCode::OK.as_u16(),
                description: None,
                value: None,
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

    use crate::handlers::table::paid::paid;
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use deadpool_diesel::postgres::{Manager, Pool, Runtime};
    use dotenvy::dotenv;
    #[tokio::test]
    async fn test_table_paid() {
        dotenv().ok();
        let manager = Manager::new(
            env::var("DATABASE_URL").expect("PORT must be set"),
            Runtime::Tokio1,
        );
        let pool = Pool::builder(manager).max_size(8).build().unwrap();
        let paid_result = paid(State(pool), Path(1)).await;
        assert!(StatusCode::OK == paid_result.0, "Не удалось оплатить стол");
    }
}
