pub mod group_model;
pub mod membership_model;
pub mod santa_model;
pub mod user_model;

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct LoginRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub name: String,
    pub uid: i32,
    pub exp: i64,
}
