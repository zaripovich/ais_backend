use crate::{models::report::Report, result::MResult};
use axum::{extract::Path, http::StatusCode, Json};

pub async fn get_report_by_id(Path(id): Path<i32>) -> (StatusCode, Json<MResult<Report>>) {
    let result = Report::get_by_id_from_db(id);
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
