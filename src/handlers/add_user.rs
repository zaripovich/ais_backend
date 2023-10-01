use axum::{http::StatusCode, Json};

use crate::models::user::NewUser;
pub async fn add_user(Json(payload): Json<NewUser>) -> (StatusCode, String) {
    let user = NewUser {
        username: payload.username,
        password: payload.password,
        role: payload.role,
    };
    if user.add_into_db() {
        (StatusCode::OK, "Success".to_string())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "User adding into db error".to_string(),
        )
    }
}
