use crate::models::table::{self, Table};
use crate::result::MResult;
use axum::{http::StatusCode, Json};

pub async fn get_updates() -> (StatusCode, Json<MResult<Vec<Table>>>) {
    let result = table::get_updates();
    match result {
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
    use crate::handlers::table::get_updates::get_updates;
    use axum::http::StatusCode;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_get_updates() {
        dotenv().ok();
        let get_updates_result = get_updates().await;
        assert!(
            StatusCode::OK == get_updates_result.0 && get_updates_result.1.value.clone().is_some(),
            "Не удалось получить обновления"
        );
    }
}
