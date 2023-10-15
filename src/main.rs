mod db;
mod handlers;
mod models;
mod result;
mod schema;
use axum::routing::{get, post};
use axum::{response::IntoResponse, Extension, Router};
use axum_login::{
    secrecy::SecretVec, AuthLayer, AuthUser, PostgresStore, RequireAuthorizationLayer,
};
use dotenvy::dotenv;
use handlers::auth::login::login;

use handlers::order::set_active::set_active_of_order;
use handlers::order::{add_order::add_order, get_order::get_order};
use handlers::product::{add_product::add_product, get_product::get_product};
use handlers::role::{add_role::add_role, get_role::get_role};
use handlers::table::get_updates::get_updates;
use handlers::user::{add_user::add_user, get_user::get_user};
use models::user::User;
use rand::Rng;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    //let session_store = RedisStore::new();
    //let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);
    let secret = rand::thread_rng().gen::<[u8; 64]>();
    let pool = PgPoolOptions::new()
        .connect(
            env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set")
                .as_str(),
        )
        .await
        .unwrap();
    let user_store = PostgresStore::<User>::new(pool);
    let auth_layer = AuthLayer::new(user_store, &secret);

    let app = Router::new()
        .route("/user/add", post(add_user))
        .route("/user/get", post(get_user))
        .route("/product/add", post(add_product))
        .route("/product/get", post(get_product))
        .route("/order/add", post(add_order))
        .route("/order/get", post(get_order))
        .route("/order/set_active", get(set_active_of_order))
        .route("/table/get_updates", get(get_updates))
        .route("/role/add", post(add_role))
        .route("/role/get", post(get_role))
        .route_layer(RequireAuthorizationLayer::<i32, User>::login())
        .route("/auth/login", post(login))
        //.layer(auth_layer)
        //.layer(session_layer)
        ;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
