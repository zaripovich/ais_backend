mod handlers;
mod models;
mod result;
mod schema;
use std::env;

use axum::routing::{delete, get, patch, post};
use axum::Router;
use deadpool_diesel::postgres::{Manager, Pool, Runtime};
use dotenvy::dotenv;

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
    let manager = Manager::new(
        env::var("DATABASE_URL").expect("PORT must be set"),
        Runtime::Tokio1,
    );
    let pool = Pool::builder(manager).max_size(8).build().unwrap();

    let app = Router::new()
        .route("/product/add", post(add_product))
        .route("/product/get/all", get(get_all))
        .route("/product/get/:product_id", get(get_product))
        .route("/order/add", post(add_order))
        .route("/order/get/:order_id", get(get_order))
        .route("/order/delete/:order_id", delete(delete_order))
        .route("/table/paid/:table_id", patch(paid))
        .route("/table/get_updates", get(get_updates))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(pool);
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let settings = format!("{}:{}", address, port);
    axum::Server::bind(&settings.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
