use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn encode_jwt(email: &str) -> String {
    let mut header = Header::default();
    header.typ = Some("JWT".to_string());
    header.alg = Algorithm::HS256;
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(24)).timestamp();
    let my_claims = Claims {
        email: email.to_string(),
        iat,
        exp,
    };
    encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret(get_secret().as_bytes()),
    )
    .unwrap()
}

pub fn decode_jwt(token: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_secret().as_bytes()),
        &Validation::default(),
    )
    .map(|v| v.claims)
    .ok()
}

fn get_secret() -> String {
    dotenv().ok();
    env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set")
}
