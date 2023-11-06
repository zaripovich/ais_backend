// @generated automatically by Diesel CLI.

diesel::table! {
    orders (id) {
        id -> Int4,
        active -> Bool,
        product_id -> Int4,
        table_id -> Int4,
        created_at -> Timestamptz,
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
    tables (id) {
        id -> Int4,
        active -> Bool,
    }
}

diesel::joinable!(orders -> products (product_id));
diesel::joinable!(orders -> tables (table_id));

diesel::allow_tables_to_appear_in_same_query!(
    orders,
    products,
    tables,
);
