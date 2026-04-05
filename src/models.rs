use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::users;
use bigdecimal::BigDecimal;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Orders {   // capital O
    pub id: i32,
    pub user_id: i32,
    pub stock_symbol: String,
    pub quantity: i32,
    pub price: BigDecimal,
    pub order_type: String,
    pub status: String,
}