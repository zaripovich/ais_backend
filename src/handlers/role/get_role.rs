use crate::{models::role::Role, result::MResult};
use axum::{http::StatusCode, Json};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct SearchParametrs {
    pub id: Option<i32>,
    pub name: Option<String>,
}

pub async fn get_role(Json(payload): Json<SearchParametrs>) -> (StatusCode, Json<MResult<Role>>) {
    let result = Role::get_from_db(payload.id, payload.name);
    match result {
        Ok(role) => {
            let r = MResult {
                status: StatusCode::OK.as_u16(),
                description: None,
                value: Some(role),
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
