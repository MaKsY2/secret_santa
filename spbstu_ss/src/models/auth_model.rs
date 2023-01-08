use serde_derive::{Deserialize, Serialize};
use crate::models::user_model::User;

#[derive(Deserialize, Clone)]
pub struct LoginRequest {
    pub name: String,
    pub password: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub name: String,
    pub uid: i32,
    pub exp: i64,
}
