use axum::{http::StatusCode, Json};

use crate::models::product::NewProduct;
pub async fn add_product(Json(payload): Json<NewProduct>) -> (StatusCode, String) {
    let product = NewProduct {
        name: payload.name,
        price: payload.price,
    };
    match product.add_into_db() {
        Ok(_) => (StatusCode::OK, "Success".to_string()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "User adding into db error".to_string(),
        ),
    }
}
