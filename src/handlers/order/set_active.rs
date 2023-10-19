use crate::models::{order::Order, table::MyTable};
use axum::{http::StatusCode, Json};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct SetParametrs {
    pub id: i32,
    pub active: bool,
}

pub async fn set_active_of_order(Json(payload): Json<SetParametrs>) -> StatusCode {
    if let Ok(_) = Order::update_into_db(payload.id, payload.active) {
        if let Ok(order) = Order::get_by_id_from_db(payload.id) {
            if let Ok(_) = MyTable::update_active_status(order.table_id, payload.active) {
                return StatusCode::OK;
            }
        }
    }
    StatusCode::INTERNAL_SERVER_ERROR
}
