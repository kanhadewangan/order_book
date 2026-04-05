// @generated automatically by Diesel CLI.




diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 10]
        stock_symbol -> Varchar,
        quantity -> Int4,
        price -> Numeric,
        #[max_length = 4]
        order_type -> Varchar,
        #[max_length = 20]
        status -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(orders, users,);
