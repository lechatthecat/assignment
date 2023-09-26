use actix_web::{HttpRequest, http::header::HeaderMap, dev::ServiceRequest};
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// For generics
pub trait RequestHeaders {
    fn get_headers(&self) -> &HeaderMap;
}

impl RequestHeaders for HttpRequest {
    fn get_headers(&self) -> &HeaderMap {
        self.headers()
    }
}

impl RequestHeaders for ServiceRequest {
    fn get_headers(&self) -> &HeaderMap {
        self.headers()
    }
}

pub fn create_token(name: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 10); // 10 days
    let claims = Claims {
        sub: name.to_owned(),
        exp: expiration.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
}

pub fn decode_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256))
}

pub fn verify <R: RequestHeaders>(req: &R)  -> Result<Claims, String>
{
    // Extract the token from the Authorization header
    if let Some(auth_header) = req.get_headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            // The token is typically prefixed by "Bearer" in the Authorization header
            let parts: Vec<&str> = auth_str.split_whitespace().collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                let token = parts[1];
                // Verify the token and decode the user information
                match self::decode_token(token) {
                    Ok(user_info) => {
                        return Ok(user_info.claims);
                    },
                    Err(err) => {
                        // Token is invalid
                        return Err(err.to_string());
                    }
                }
            }
        }
    }
    return Err("Header Authorization is not found".to_owned());
}