use crate::{models::report::Report, result::MResult};
use axum::{extract::Path, http::StatusCode, Json};

pub async fn get_reports_by_role(
    Path(role): Path<i32>,
) -> (StatusCode, Json<MResult<Vec<Report>>>) {
    let result = Report::get_by_role_from_db(role);
    match result {
        Ok(report) => {
            let r = MResult {
                status: 200,
                description: None,
                value: Some(report),
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
