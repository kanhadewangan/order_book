use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::{NewUser, RegisterRequest, LoginRequest, User};
use crate::schema::users::dsl::*;
use crate::auth::create_jwt;



pub async fn register_user(
    pool: web::Data<DbPool>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Check if email already exists
    let existing_user = users
        .filter(email.eq(&req.email))
        .first::<User>(&mut conn)
        .optional()
        .expect("Error checking for existing user");

    if existing_user.is_some() {
        return HttpResponse::BadRequest().body("Email already registered");
    }

    // Hash the password
    let hashed_password: String = hash(&req.password, DEFAULT_COST).expect("Error hashing password");

    let new_user = NewUser {
        username: req.username.clone(),
        email: req.email.clone(),
        password_hash: hashed_password,
    };

    let result = diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().body("User registered"),
        Err(_) => HttpResponse::InternalServerError().body("Error registering user"),
    }
}

pub async fn login_user(
    pool: web::Data<DbPool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let user = users
        .filter(email.eq(&req.email))
        .first::<User>(&mut conn)
        .optional()
        .expect("Error fetching user");

    match user {
        Some(user) => {
            if verify(&req.password, &user.password_hash).unwrap_or(false) {
                let token = create_jwt(user.id, user.email.clone());
                HttpResponse::Ok().json(serde_json::json!({ "token": token }))
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        None => HttpResponse::Unauthorized().body("Invalid credentials"),
    }

}
