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
