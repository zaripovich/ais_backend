use crate::{
    models::{order::Order, table::MyTable},
    result::MResult,
};
use axum::{http::StatusCode, Json};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct SetParametrs {
    pub id: i32,
    pub active: bool,
}

pub async fn set_active_of_order(
    Json(payload): Json<SetParametrs>,
) -> (StatusCode, Json<MResult<String>>) {
    if let Ok(_) = Order::update_into_db(payload.id, payload.active) {
        if let Ok(order) = Order::get_by_id_from_db(payload.id) {
            if let Ok(_) = MyTable::update_active_status(order.table_id, payload.active) {
                let r = MResult {
                    status: StatusCode::OK.as_u16(),
                    description: Some("Sucess".to_string()),
                    value: None,
                };
                return (StatusCode::OK, Json(r));
            }
        }
    }
    let r = MResult {
        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        description: Some("Database error!".to_string()),
        value: None,
    };
    (StatusCode::INTERNAL_SERVER_ERROR, Json(r))
}
