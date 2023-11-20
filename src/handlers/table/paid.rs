use crate::{models::table::MyTable, result::MResult};
use axum::{extract::Path, http::StatusCode, Json};

pub async fn paid(Path(table_id): Path<i32>) -> (StatusCode, Json<MResult<String>>) {
    let result = MyTable::paid(table_id);
    match result {
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

    use crate::handlers::table::paid::paid;
    use axum::extract::Path;
    use axum::http::StatusCode;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_table_paid() {
        dotenv().ok();

        let paid_result = paid(Path(1)).await;
        assert!(StatusCode::OK == paid_result.0, "Не удалось оплатить стол");
    }
}
