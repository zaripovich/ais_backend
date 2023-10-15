use crate::models::user::User;
use crate::result::MResult;
use axum::{http::StatusCode, Json};
use axum_login::secrecy::SecretVec;
use axum_login::AuthUser;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

impl AuthUser<i32> for User {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password.clone().into())
    }
}

pub async fn login(Json(login_data): Json<LoginData>) -> (StatusCode, Json<MResult<User>>) {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(login_data.password.as_str());
    let password_hash = hasher.result_str();
    if let Ok(user) = User::get_by_username_from_db(login_data.username) {
        if password_hash != user.password {
            return (
                StatusCode::OK,
                Json(MResult {
                    status: 200,
                    description: None,
                    value: Some(user),
                }),
            );
        }
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(MResult {
            status: 400,
            description: Some("Incorrect password".to_string()),
            value: None,
        }),
    )
}
