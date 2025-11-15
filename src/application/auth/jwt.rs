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
    let secret = get_secret();
    encode_jwt_internal(email, &secret)
}

pub fn decode_jwt(token: &str) -> Option<Claims> {
    let secret = get_secret();
    decode_jwt_internal(token, &secret)
}

/// 内部実装: シークレットキーを指定してJWTをエンコード
fn encode_jwt_internal(email: &str, secret: &str) -> String {
    let header = Header {
        typ: Some("JWT".to_string()),
        alg: Algorithm::HS256,
        ..Default::default()
    };
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
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

/// 内部実装: シークレットキーを指定してJWTをデコード
fn decode_jwt_internal(token: &str, secret: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|v| v.claims)
    .ok()
}

/// テスト用: シークレットキーを指定してJWTをエンコード
#[cfg(test)]
pub fn encode_jwt_with_secret(email: &str, secret: &str) -> String {
    encode_jwt_internal(email, secret)
}

/// テスト用: シークレットキーを指定してJWTをデコード
#[cfg(test)]
pub fn decode_jwt_with_secret(token: &str, secret: &str) -> Option<Claims> {
    decode_jwt_internal(token, secret)
}

fn get_secret() -> String {
    dotenv().ok();
    env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set")
}
