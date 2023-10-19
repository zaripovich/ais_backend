use crate::{models::order::Order, result::MResult};
use axum::{extract::Path, http::StatusCode, Json};

pub async fn get_order(Path(order_id): Path<i32>) -> (StatusCode, Json<MResult<Order>>) {
    let result = Order::get_by_id_from_db(order_id);
    match result {
        Ok(order) => {
            let r = MResult {
                status: StatusCode::OK.as_u16(),
                description: None,
                value: Some(order),
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
