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