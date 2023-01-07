const JWT_SECRET: &[u8] = b"secret";
const JWT_ERROR: &str = "Error in JWT";

use diesel::result::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::response::status::NotFound;
use rocket_contrib::json::Json;
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};

use spbstu_ss::models::{Claims, LoginResponse};

pub fn create_jwt(
    uid: i32,
    name: String,
) -> std::result::Result<Json<LoginResponse>, NotFound<String>> {
    let claims = Claims {
        name: name.to_string(),
        uid: uid.to_owned(),
    };
    let header = Header::new(Algorithm::HS512);
    match encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)) {
        Ok(t) => Ok(Json(LoginResponse { token: t })),
        Err(err) => {
            panic!("{}", err.to_string())
        }
    }
}

pub fn authorise((name, headers): (String, HeaderMap<HeaderValue>)) -> Result<String> {}

pub fn extract_jwt(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(Error::NoAuthHeaderError),
    };
}
