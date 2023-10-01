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
                status: 200,
                description: None,
                value: Some(role),
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
