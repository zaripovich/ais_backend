mod db;
mod handlers;
mod models;
mod result;
mod schema;
use axum::{
    routing::{get, post},
    Router,
};

use handlers::{
    add_report::add_report, get_report::get_report_by_id, get_reports_by_role::get_reports_by_role,
};
use handlers::{add_role::add_role, get_role::get_role};
use handlers::{add_user::add_user, get_user::get_user};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/user/add", post(add_user))
        .route("/user/get", post(get_user))
        .route("/report/add", post(add_report))
        .route("/report/id/:id", get(get_report_by_id))
        .route("/report/role/:role", get(get_reports_by_role))
        .route("/role/add", post(add_role))
        .route("/role/get", post(get_role));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
