use axum::{http::StatusCode, Json};

use crate::models::order::NewOrder;

use serde::Deserialize;
#[derive(Deserialize)]
pub struct AddParametrs {
    pub product_id: i32,
    pub table_id: i32,
}

pub async fn add_order(Json(payload): Json<AddParametrs>) -> (StatusCode, String) {
    let order = NewOrder {
        active: true,
        product_id: payload.product_id,
        table_id: payload.table_id,
    };
    match order.add_into_db() {
        Ok(_) => (StatusCode::OK, "Success".to_string()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "User adding into db error".to_string(),
        ),
    }
}
