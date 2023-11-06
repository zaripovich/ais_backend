mod handlers_test;
use dotenvy::dotenv;
use handlers_test::test_all;
mod db;
mod handlers;
mod models;
mod result;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();
    test_all().await;
}
