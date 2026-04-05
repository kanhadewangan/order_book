
use  actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::schema::orders::dsl::*;
use crate::db::DbPool;
use crate::models::{NewOrder, Orders};

pub async fn create_limit(
    pool: web::Data<DbPool>,
    new_order: web::Json<NewOrder>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Check if the stock symbol already exists in the orders table
    let existing_order = orders
        .filter(stock_symbol.eq(&new_order.stock_symbol))
        .first::<Orders>(&mut conn)
        .optional()
        .expect("Error checking existing orders");

    if existing_order.is_some() {
        return HttpResponse::BadRequest().body("Stock symbol already exists in orders");
    }

    // Insert the new order into the database
    let result = diesel::insert_into(orders)
        .values(&new_order.into_inner())
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().body("Order created"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating order"),
    }
}

pub  async fn get_limits(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let result = orders
        .select(Orders::as_select())
        .load(&mut conn);

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching limits"),
    }
}