use axum::{http::StatusCode, Json};
use crypto::{digest::Digest, sha3::Sha3};

use crate::models::user::NewUser;
pub async fn add_user(Json(payload): Json<NewUser>) -> (StatusCode, String) {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(payload.password.as_str());
    let password_hash = hasher.result_str();
    let user = NewUser {
        username: payload.username,
        password: password_hash,
        role: payload.role,
    };
    match user.add_into_db() {
        Ok(_) => (StatusCode::OK, "Success".to_string()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "User adding into db error".to_string(),
        ),
    }
}
