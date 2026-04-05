use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::models::{Claims, User};


const secret_key:[u8; 32] =  [0; 32]; // In production, use a secure key from env variable or config


pub fn create_jwt(id: i32,email: String) -> String{
    let expire = Utc::now().checked_add_signed(Duration::hours(24)).expect("valid timestamp").timestamp() as usize;
    let claims = Claims {
        sub: id,
        email: email,
        exp: expire,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(&secret_key))
        .expect("Failed to create JWT")
}

pub fn decode_jwt(token: &str) -> Option<Claims> {
    let validation = Validation::default();
    match decode::<Claims>(token, &DecodingKey::from_secret(&secret_key), &validation) {
        Ok(token_data) => Some(token_data.claims),
        Err(_) => None,
    }
}

