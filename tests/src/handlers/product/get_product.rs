use crate::{models::product::Product, result::MResult};
use axum::{extract::Path, http::StatusCode, Json};

pub async fn get_product(Path(product_id): Path<i32>) -> (StatusCode, Json<MResult<Product>>) {
    let result = Product::get_by_id_from_db(product_id);
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
