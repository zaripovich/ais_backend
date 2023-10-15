// @generated automatically by Diesel CLI.

diesel::table! {
    orders (id) {
        id -> Int4,
        active -> Bool,
        product_id -> Int4,
        table_id -> Int4,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 30]
        name -> Varchar,
        price -> Int4,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 30]
        name -> Varchar,
    }
}

diesel::table! {
    tables (id) {
        id -> Int4,
        active -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 30]
        username -> Varchar,
        password -> Text,
        role -> Int4,
    }
}

diesel::joinable!(orders -> products (product_id));
diesel::joinable!(orders -> tables (table_id));
diesel::joinable!(users -> roles (role));

diesel::allow_tables_to_appear_in_same_query!(
    orders,
    products,
    roles,
    tables,
    users,
);
