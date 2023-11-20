mod db;
mod handlers;
mod models;
mod result;
mod schema;
mod tests;
use std::env;

use axum::routing::{delete, get, patch, post, put};
use axum::Router;
use dotenvy::dotenv;
use handlers::order::set_active::set_active_of_order;
use handlers::order::{add_order::add_order, delete_order::delete_order, get_order::get_order};
use handlers::product::get_all::get_all;
use handlers::product::{add_product::add_product, get_product::get_product};
use handlers::table::get_updates::get_updates;
use handlers::table::paid::paid;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let address = env::var("ADDRESS").expect("ADDRESS must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let app = Router::new()
        .route("/product/add", post(add_product))
        .route("/product/get/all", get(get_all))
        .route("/product/get/:product_id", get(get_product))
        .route("/order/add", post(add_order))
        .route("/order/get/:order_id", get(get_order))
        .route("/order/delete/:order_id", delete(delete_order))
        .route("/order/set_active", put(set_active_of_order))
        .route("/table/paid/:table_id", patch(paid))
        .route("/table/get_updates", get(get_updates))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());
    let settings = format!("{}:{}", address, port);
    axum::Server::bind(&settings.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
