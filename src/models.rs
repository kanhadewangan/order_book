use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use bigdecimal::BigDecimal;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]  // gives better errors
pub struct Orders {
    pub id: i32,
    pub user_id: i32,
    pub stock_symbol: String,
    pub quantity: i32,
    pub price: BigDecimal,
    pub order_type: String,
    pub status: String,
    pub created_at: Option<chrono::NaiveDateTime>,  // nullable in schema
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::orders)]
pub struct NewOrder {
    pub user_id: i32,
    pub stock_symbol: String,
    pub quantity: i32,
    pub price: BigDecimal,
    pub order_type: String,
    pub status: String,
}




use crate::schema::users;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

// Request bodies
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// JWT Claims
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,       // user id
    pub email: String,
    pub exp: usize,     // expiry timestamp
}