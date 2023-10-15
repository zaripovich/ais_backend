use crate::{models::product::Product, result::MResult};
use axum::{http::StatusCode, Json};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct SearchParametrs {
    pub id: i32,
}

pub async fn get_product(
    Json(payload): Json<SearchParametrs>,
) -> (StatusCode, Json<MResult<Product>>) {
    let result = Product::get_by_id_from_db(payload.id);
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
