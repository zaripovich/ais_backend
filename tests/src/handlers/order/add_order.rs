use crate::{models::order::NewOrder, result::MResult};
use axum::{http::StatusCode, Json};
use chrono::Local;

use serde::Deserialize;
#[derive(Deserialize)]
pub struct AddParametrs {
    pub product_id: i32,
    pub table_id: i32,
}

pub async fn add_order(Json(payload): Json<AddParametrs>) -> (StatusCode, Json<MResult<i32>>) {
    let order = NewOrder {
        active: true,
        product_id: payload.product_id,
        table_id: payload.table_id,
        created_at: Local::now().naive_local(),
    };
    match order.add_into_db() {
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
