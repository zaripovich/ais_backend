use axum::{http::StatusCode, Json};

use crate::models::role::NewRole;
pub async fn add_role(Json(payload): Json<NewRole>) -> (StatusCode, String) {
    let role = NewRole { name: payload.name };
    match role.add_into_db() {
        Ok(_) => (StatusCode::OK, "Success".to_string()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "User adding into db error".to_string(),
        ),
    }
}
