#[cfg(test)]
mod tests{
    
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
    use dotenvy::dotenv;
    
    #[tokio::test]
    async fn test_add_order(){
        dotenv().ok();
        let product_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(29)
            .map(char::from)
            .collect();
        let test_product = NewProduct {
            name: product_name.clone(),
            price: 1000,
        };
        let add_product_result = add_product(Json(test_product)).await;

        assert!(
            StatusCode::OK == add_product_result.0,
            "Не удалось добавить продукт"
        );
        let product_id = add_product_result.1.value.unwrap();

        let parametrs = AddParametrs {
            product_id: product_id,
            table_id: 1,
        };
        let add_order_result = add_order(Json(parametrs)).await;

        assert!(StatusCode::OK == add_order_result.0 && add_order_result.1.value.is_some(), "Не удалось добавить заказ");
    }

    #[tokio::test]
    async fn test_delete_order(){
        dotenv().ok();
        let product_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(29)
            .map(char::from)
            .collect();
        let test_product = NewProduct {
            name: product_name.clone(),
            price: 1000,
        };
        let add_product_result = add_product(Json(test_product)).await;

        assert!(
            StatusCode::OK == add_product_result.0,
            "Не удалось добавить продукт"
        );
        let product_id = add_product_result.1.value.unwrap();

        let parametrs = AddParametrs {
            product_id: product_id,
            table_id: 1,
        };
        let add_order_result = add_order(Json(parametrs)).await;
        assert!(StatusCode::OK == add_order_result.0 && add_order_result.1.value.is_some(), "Не удалось добавить заказ");

        let order_id = add_order_result.1.value.unwrap();
        let delete_result = delete_order(Path(order_id)).await;
        assert!(StatusCode::OK == delete_result, "Не удалось удалить заказ");
    }

    #[tokio::test]
    async fn test_get_order() {
        dotenv().ok();

        let product_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(29)
            .map(char::from)
            .collect();
        let test_product = NewProduct {
            name: product_name.clone(),
            price: 1000,
        };
        let add_product_result = add_product(Json(test_product)).await;

        assert!(
            StatusCode::OK == add_product_result.0,
            "Не удалось добавить продукт"
        );
        let product_id = add_product_result.1.value.unwrap();

        let parametrs = AddParametrs {
            product_id: product_id,
            table_id: 1,
        };
        let add_order_result = add_order(Json(parametrs)).await;

        assert!(StatusCode::OK == add_order_result.0, "Не удалось добавить заказ");
        let order_id = add_order_result.1.value.unwrap();

        let path = Path(order_id as i32);
        let get_result = get_order(path).await;
        assert!(
            StatusCode::OK == get_result.0 && get_result.1.value.clone().unwrap().product_id == product_id,
            "Не удалось получить заказ"
        );
    }

    #[tokio::test]
    async fn test_add_product(){
        dotenv().ok();

        let product_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(29)
            .map(char::from)
            .collect();
        let test_product = NewProduct {
            name: product_name.clone(),
            price: 1000,
        };
        let add_result = add_product(Json(test_product)).await;
        assert!(
            StatusCode::OK == add_result.0,
            "Не удалось добавить продукт"
        );
    }

    #[tokio::test]
    async fn test_get_product() {
        dotenv().ok();

        let product_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(29)
            .map(char::from)
            .collect();
        let test_product = NewProduct {
            name: product_name.clone(),
            price: 1000,
        };
        let add_result = add_product(Json(test_product)).await;

        assert!(
            StatusCode::OK == add_result.0,
            "Не удалось добавить продукт"
        );
        let product_id = add_result.1.value.unwrap();
        let path = Path(product_id as i32);
        let get_result = get_product(path).await;
        assert!(
            StatusCode::OK == get_result.0 && get_result.1.value.is_some(),
            "Не удалось получить продукт"
        );
        assert!(
            get_result.1.value.clone().unwrap().name == product_name,
            "Не удалось получить продукт"
        );
    }

    #[tokio::test]
    async fn test_get_all_product() {
        dotenv().ok();

        let get_results = get_all().await;
        assert!(
            StatusCode::OK == get_results.0 && get_results.1.value.clone().is_some(),
            "Не удалось получить все продукты"
        );
    }

    #[tokio::test]
    async fn test_table_paid() {
        dotenv().ok();

        let paid_result = paid(Path(1)).await;
        assert!(StatusCode::OK == paid_result.0, "Не удалось оплатить стол");
    }

    #[tokio::test]
    async fn test_get_updates() {
        dotenv().ok();

        let get_updates_result = get_updates().await;
        assert!(
            StatusCode::OK == get_updates_result.0 && get_updates_result.1.value.clone().is_some(),
            "Не удалось получить обновления"
        );
    }
}
