use crate::{models::user::User, result::MResult};
use axum::{http::StatusCode, Json};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct SearchParametrs {
    pub id: i32,
}

pub async fn get_user(Json(payload): Json<SearchParametrs>) -> (StatusCode, Json<MResult<User>>) {
    let result = User::get_by_id_from_db(payload.id);
    match result {
        Ok(user) => {
            let r = MResult {
                status: StatusCode::OK.as_u16(),
                description: None,
                value: Some(user),
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
