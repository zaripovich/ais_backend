use crate::models::order::Order;
use axum::{extract::Path, http::StatusCode};

pub async fn delete_order(Path(order_id): Path<i32>) -> StatusCode {
    if let Ok(_) = Order::delete_from_db(order_id) {
        return StatusCode::OK;
    }
    StatusCode::INTERNAL_SERVER_ERROR
}
