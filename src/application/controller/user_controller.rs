use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rocket::http::{hyper::StatusCode, Status};
use rocket_contrib::json::Json;

use crate::{
    application::api_model::user_api_model::{AuthRequest, AuthResponse},
    infrastructure::user_repository,
};
use argon2::{self, Config, Variant, Version};
use chrono::{Duration, Utc};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    email: String,
    iat: i64,
    exp: i64,
}

#[post("/signup", format = "json", data = "<auth_req>")]
pub fn post_signup(auth_req: Json<AuthRequest>) -> Result<Json<AuthResponse>, Status> {
    let exists_user = user_repository::exists_user(auth_req.email.clone());
    if exists_user {
        return Err(Status::Conflict);
    }

    let salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let config = Config {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 1,
        lanes: 4,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };
    let hashed_password =
        argon2::hash_encoded(&auth_req.password.as_bytes(), &salt.as_bytes(), &config).unwrap();

    user_repository::post_user(auth_req.email.clone(), hashed_password, salt);

    let mut header = Header::default();
    header.typ = Some("JWT".to_string());
    header.alg = Algorithm::HS256;
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(24)).timestamp();
    let my_claims = Claims {
        email: auth_req.email.clone(),
        iat,
        exp,
    };
    let jwt_token = encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret("test".as_bytes()),
    )
    .unwrap();

    Ok(Json(AuthResponse { token: jwt_token }))
}
