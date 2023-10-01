use crate::{models::user::User, result::MResult};
use axum::{http::StatusCode, Json};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct SearchParametrs {
    pub id: i32,
}

pub async fn get_user(Json(payload): Json<SearchParametrs>) -> (StatusCode, Json<MResult<User>>) {
    let result = User::get_from_db(payload.id);
    match result {
        Ok(user) => {
            let r = MResult {
                status: 200,
                description: None,
                value: Some(user),
            };
            (StatusCode::OK, Json(r))
        }
        Err(err) => {
            let r = MResult {
                status: 400,
                description: Some(err.to_string()),
                value: None,
            };
            (StatusCode::OK.into(), Json(r))
        }
    }
}
