const JWT_SECRET: &[u8] = b"secret";
const JWT_ERROR: &str = "Error in JWT";

use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use spbstu_ss::models::auth_model::Claims;

use chrono::Utc;

pub fn create_jwt(uid: i32, name: &String) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        name: name.to_string(),
        uid: uid.to_owned(),
        exp: expiration,
    };
    let header = Header::new(Algorithm::HS512);
    return encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)).unwrap();
}

pub fn extract_jwt(token: String) -> Result<Claims, Error> {
    return match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => Err(err),
    };
}
