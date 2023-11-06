use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use handlers::order::{
    add_order::{add_order, AddParametrs},
    delete_order::delete_order,
    get_order::get_order,
};
use handlers::product::{add_product::add_product, get_all::get_all, get_product::get_product};
use models::product::NewProduct;
use rand::{distributions::Alphanumeric, Rng};

use crate::{
    handlers::{
        self,
        table::{get_updates::get_updates, paid::paid},
    },
    models,
};

pub async fn test_all() {
    let product_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(29)
        .map(char::from)
        .collect();
    let product_id = test_add_product(product_name.clone()).await;
    _ = test_get_product(product_id, product_name).await;
    _ = test_get_all_product().await;
    let order_id = test_add_order(product_id).await;
    _ = test_get_order(order_id).await;
    _ = test_delete_order(order_id).await;
    _ = test_get_updates().await;
    _ = test_table_paid().await;
}

async fn test_add_order(product_id: i32) -> i32 {
    println!("Тестирование добавления заказа");
    let parametrs = AddParametrs {
        product_id: product_id,
        table_id: 1,
    };
    let add_result = add_order(Json(parametrs)).await;

    assert!(StatusCode::OK == add_result.0, "Не удалось добавить заказ");
    println!("Успешно\n");
    let order_id = add_result.1.value.unwrap();
    order_id as i32
}

async fn test_delete_order(order_id: i32) {
    println!("Тестирование удаления заказа");
    let delete_result = delete_order(Path(order_id)).await;
    assert!(StatusCode::OK == delete_result, "Не удалось удалить заказ");
    println!("Успешно\n");
}

async fn test_get_order(order_id: i32) {
    println!("Тестирование получения заказа");
    let path = Path(order_id as i32);
    let get_result = get_order(path).await;
    assert!(
        StatusCode::OK == get_result.0 && get_result.1.value.clone().is_some(),
        "Не удалось получить заказ"
    );
    println!("Успешно\n");
}

async fn test_add_product(product_name: String) -> i32 {
    println!("Тестирование добавления продукта");
    let test_product = NewProduct {
        name: product_name.clone(),
        price: 1000,
    };
    let add_result = add_product(Json(test_product)).await;

    assert!(
        StatusCode::OK == add_result.0,
        "Не удалось добавить продукт"
    );
    println!("Успешно\n");
    let product_id = add_result.1.value.unwrap();

    product_id as i32
}

async fn test_get_product(product_id: i32, product_name: String) {
    println!("Тестирование получения продукта");
    let path = Path(product_id as i32);
    let get_result = get_product(path).await;
    assert!(
        StatusCode::OK == get_result.0 && get_result.1.value.clone().unwrap().name == product_name,
        "Не удалось получить продукт"
    );
    println!("Успешно\n");
}

async fn test_get_all_product() {
    println!("Тестирование получения всех продуктов");
    let get_results = get_all().await;
    assert!(
        StatusCode::OK == get_results.0 && get_results.1.value.clone().is_some(),
        "Не удалось получить все продукты"
    );
    println!("Успешно\n");
}

async fn test_table_paid() {
    println!("Тестирование оплаты стола");
    let paid_result = paid(Path(1)).await;
    assert!(StatusCode::OK == paid_result.0, "Не удалось оплатить стол");
    println!("Успешно\n");
}

async fn test_get_updates() {
    println!("Тестирование получения обновлений");
    let get_updates_result = get_updates().await;
    assert!(
        StatusCode::OK == get_updates_result.0 && get_updates_result.1.value.clone().is_some(),
        "Не удалось получить обновления"
    );
    println!("Успешно\n");
}
