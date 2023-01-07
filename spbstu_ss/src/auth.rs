const JWT_SECRET: &[u8] = b"secret";
const JWT_ERROR: &str = "Error in JWT";

use rocket_contrib::json::Json;
use serde_json::*;
use spbstu_ss::models::{Claims, LoginResponse};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

pub fn create_jwt(uid: i32, name: String) -> std::result::Result<Json<LoginResponse>, String> {
    let claims = Claims {
        name: name.to_string(),
        uid: uid.to_owned(),
    };
    let header = Header::new(Algorithm::HS512);
    match encode
    (
        &header,
        &claims, 
        &EncodingKey::from_secret(JWT_SECRET)
    ) {
        Ok(t) => Ok(Json(LoginResponse{token: t})),
        Err(_) => Err(JWT_ERROR.to_string()),
    }
}