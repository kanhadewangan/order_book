use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::{NewOrder, Orders};
use crate::schema::orders::dsl::*;

// GET /orders
pub async fn get_orders(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let result = orders
        .select(Orders::as_select())  // ← fix is here
        .load(&mut conn);

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching orders"),
    }
}

// POST /orders
pub async fn create_order(
    pool: web::Data<DbPool>,
    new_order: web::Json<NewOrder>,  // use NewOrder, not Orders
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let result = diesel::insert_into(orders)
        .values(&new_order.into_inner())
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().body("Order created"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating order"),
    }
}