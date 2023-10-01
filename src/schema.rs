// @generated automatically by Diesel CLI.

diesel::table! {
    reports (id) {
        id -> Int4,
        description -> Text,
        role -> Int4,
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
    users (id) {
        id -> Int4,
        #[max_length = 30]
        username -> Varchar,
        password -> Text,
        role -> Int4,
    }
}

diesel::joinable!(reports -> roles (role));
diesel::joinable!(users -> roles (role));

diesel::allow_tables_to_appear_in_same_query!(
    reports,
    roles,
    users,
);
