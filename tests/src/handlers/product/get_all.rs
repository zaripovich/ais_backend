use crate::{models::product::Product, result::MResult};
use axum::{http::StatusCode, Json};

pub async fn get_all() -> (StatusCode, Json<MResult<Vec<Product>>>) {
    let result = Product::get_all_from_db();
    match result {
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
