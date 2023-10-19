mod db;
mod handlers;
mod models;
mod result;
mod schema;
use axum::routing::{get, post, put};
use axum::Router;
use dotenvy::dotenv;
use handlers::order::set_active::set_active_of_order;
use handlers::order::{add_order::add_order, get_order::get_order};
use handlers::product::{add_product::add_product, get_product::get_product};
use handlers::table::get_updates::get_updates;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/product/add", post(add_product))
        .route("/product/get/:product_id", get(get_product))
        .route("/order/add", post(add_order))
        .route("/order/get/:order_id", get(get_order))
        .route("/order/set_active", put(set_active_of_order))
        .route("/table/get_updates", get(get_updates));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
