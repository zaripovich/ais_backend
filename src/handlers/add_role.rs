use axum::{http::StatusCode, Json};

use crate::models::role::NewRole;
pub async fn add_role(Json(payload): Json<NewRole>) -> (StatusCode, String) {
    let role = NewRole { name: payload.name };
    if role.add_into_db() {
        (StatusCode::OK, "Success".to_string())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "User adding into db error".to_string(),
        )
    }
}
