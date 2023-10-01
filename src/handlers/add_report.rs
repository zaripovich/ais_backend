use axum::{http::StatusCode, Json};

use crate::models::report::NewReport;
pub async fn add_report(Json(payload): Json<NewReport>) -> (StatusCode, String) {
    let report = NewReport {
        description: payload.description,
        role: payload.role,
    };
    if report.add_into_db() {
        (StatusCode::OK, "Success".to_string())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Report adding into db error".to_string(),
        )
    }
}
