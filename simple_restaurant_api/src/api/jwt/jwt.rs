use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_token(username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now() + Duration::from_secs(60 * 60); // 1 hour
    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
}

pub fn _decode_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256))
}
